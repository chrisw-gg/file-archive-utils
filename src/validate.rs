use crate::asset::{Assets};
use crate::crypto::{Crypto, FileHash};
use crate::directory::{Directory};
use crate::meta::{MetaData, MetaDataError};

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
	HashMatches,
}

enum Invalid {
	MissingMetadata,
	MissingMetadataHistory { metadata: MetaData },
	TimestampMismatch { metadata: MetaData },
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
			Invalid::MissingMetadataHistory { metadata } | Invalid::TimestampMismatch { metadata } => {
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

		// TODO: Check if metadata is later than file timestamp...

		if !options.contents {

			if last_modified_time != metadata_file_hash.last_modified_time {
				// options.verbose.then(|| println!("{} -> timestamp mismatch file.time({}) != metadata.time({})", file.path().display(), last_modified_time, metadata.file_hash.last_modified_time));
				return Ok(Result::Invalid(Invalid::TimestampMismatch { metadata: metadata }));
			}

		}

		let file_hash= Crypto::sha256(file)?;

		if file_hash.sha256 != metadata_file_hash.sha256 {
			// options.verbose.then(|| println!("{} -> hash mismatch file.hash({:?}) != metadata.hash({:?})", file.path().display(), file_hash, metadata.file_hash));
			return Ok(Result::Invalid(Invalid::HashMismatch { metadata, file_hash }));
		}

		let result = Result::Valid(Valid::HashMatches);
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
			Valid::TimestampMatches => "timestamp matches",
			Valid::HashMatches => "hash matches",
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
			Invalid::TimestampMismatch { .. } => "timestamp mismatch",
			Invalid::HashMismatch { .. } => "hash mismatch",
		}
	}

}
