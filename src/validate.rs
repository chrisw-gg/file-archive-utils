use crate::asset::{Assets};
use crate::crypto::{Crypto, FileHash};
use crate::directory::{Directory};
use crate::meta::{MetaData, MetaDataError, MetadataTimestampComparison};

use color_print::cprintln;
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};
use uuid::{Uuid};

pub struct ValidateOptions {
	pub contents: bool,
	pub dry_run: bool,
	pub log_level: LogLevel,
}

pub enum LogLevel {
	Minimal,
	Default,
	Verbose,
}

pub struct Validate {

}

enum Result {
	Valid(Valid),
	Invalid(Invalid),
}

enum Valid {
	TimestampMatches,
	HashAndTimestampMatches,
}

enum Invalid {
	MissingMetadata,
	MissingMetadataHistory { metadata: MetaData },
	FileModified { metadata: MetaData },
	HashMismatch { metadata: MetaData, file_hash: FileHash },
}

impl Validate {

	pub fn validate_and_update_metadata(assets: &Assets, options: &ValidateOptions) {

		cprintln!("\n<cyan>Begin</cyan>\n");

		for (id, file) in assets.file_map.iter() {

			match Validate::validate_and_update_metadata_file(file, options) {
				Ok(result) => result.print_line(id, options),
				Err(err) => {
					cprintln!("<red>{} -> {}</red>", id.to_string_lossy(), err.to_string());
				}

			}
			
		}

		cprintln!("\n<cyan>End</cyan>\n");

	}

	fn validate_and_update_metadata_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<Result, Box<dyn Error>> {

		let validation_result = Validate::validate_file(file, options)?;

		match validation_result {
			Result::Valid { .. } => {}
			Result::Invalid(ref invalid) => Self::update_metdata_file(file, options, invalid)?,
		};

		Ok(validation_result)

	}

	fn update_metdata_file(file: &DirEntry, options: &ValidateOptions, result: &Invalid) -> std::result::Result<(), Box<dyn Error>> {

		let metadata= match result {
			Invalid::MissingMetadata => {
				MetaData::new(Uuid::new_v4()).with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::MissingMetadataHistory { metadata } | Invalid::FileModified { metadata } => {
				metadata.with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::HashMismatch { metadata, file_hash } => {
				metadata.with_file_hash(file_hash.clone())
			}
		};

		if !options.dry_run {
			// Don't update if there is a hash mismatch, this most likely means that the one of the files has been corrupted!
			match result {
				Invalid::HashMismatch { .. } => (),
				_ => MetaData::update(file, &metadata)?,
			}
		}

		Ok(())

	}

	fn validate_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<Result, Box<dyn Error>> {

		let last_modified_time = Directory::last_modified_time(file);

		let metadata = match MetaData::read(file) {
			Ok(metadata) => metadata,
			Err(metadata_error) => {
				match metadata_error {
					MetaDataError::NotFound => {
						return Ok(Result::Invalid(Invalid::MissingMetadata));
					}
					MetaDataError::ReadError { error } => {
						let msg = match options.log_level {
							LogLevel::Verbose => format!("{} -> read error:\n{}", file.path().display(), error),
							_ => format!("io error reading metadata file"),
						};
						return Err(msg.into());
					}
					MetaDataError::ParseError { error } => {
						let msg = match options.log_level {
							LogLevel::Verbose => format!("{} -> parse error:\n{}", file.path().display(), error),
							_ => format!("parse error reading metadata file"),
						};
						return Err(msg.into());
					}
				}
			}
		};

		let Some(metadata_file_hash) = metadata.last_file_hash() else {
			return Ok(Result::Invalid(Invalid::MissingMetadataHistory { metadata: metadata }));
		};

		match MetaData::compare_timestamp(&metadata_file_hash, last_modified_time) {
			MetadataTimestampComparison::Error => return Err("Metadata timestamp is later than file timestamp ???".into()),
			MetadataTimestampComparison::FileModified => return Ok(Result::Invalid(Invalid::FileModified { metadata: metadata })),
			MetadataTimestampComparison::Equal => {
				// If we are not doing a full validation (checking contents as well as timestamp) then we can exit early
				if !options.contents {
					return Ok(Result::Valid(Valid::TimestampMatches));
				}
			}
		}

		let file_hash= Crypto::sha256(file)?;

		if file_hash.sha256 != metadata_file_hash.sha256 {
			return Ok(Result::Invalid(Invalid::HashMismatch { metadata, file_hash }));
		}

		let result = Result::Valid(Valid::HashAndTimestampMatches);
		Ok(result)

	}

}

impl Result {

	fn print_line(&self, id: &PathBuf, options: &ValidateOptions) {

		match self {
			Result::Valid(valid) => valid.print_line(id, options),
			Result::Invalid(invalid) => invalid.print_line(id, options),
		}

	}

}

impl Valid {

	fn print_line(&self, id: &PathBuf, options: &ValidateOptions) {
		match options.log_level {
			LogLevel::Verbose => cprintln!("<green>{} -> {}</green>", id.to_string_lossy(), self.to_string()),
			_ => return // Print nothing if not verbose
		}
	}

	fn to_string(&self) -> &str {
		match self {
			Valid::TimestampMatches => "✓ timestamp",
			Valid::HashAndTimestampMatches => "✓ hash ✓ timestamp",
		}
	}

}

impl Invalid {

	fn print_line(&self, id: &PathBuf, _options: &ValidateOptions) {
		match self {
			Invalid::HashMismatch { .. } => cprintln!("<bright-red>{} -> {}<bright-red>", id.to_string_lossy(), self.to_string()),
			_ => cprintln!("<cyan>{} -> {}</cyan>", id.to_string_lossy(), self.to_string()),
		}
		
	}

	fn to_string(&self) -> &str {
		match self {
			Invalid::MissingMetadata => "missing metadata",
			Invalid::MissingMetadataHistory { .. } => "missing metadata history",
			Invalid::FileModified { .. } => "file modified",
			Invalid::HashMismatch { .. } => "hash mismatch",
		}
	}

}
