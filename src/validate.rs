use crate::asset::{Assets};
use crate::crypto::{Crypto, FileHash};
use crate::directory::{Directory};
use crate::meta::{MetaData, MetaDataError};

use std::error::{Error};
use std::fs::{DirEntry};
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

		for (id, file) in assets.file_map.iter() {

			let result = Validate::validate_and_update_metadata_file(file, options);

			let string_result = match result {
				Ok(option) => {
					match option {
						Some(msg) => Some(msg),
						None => None,
					}
				},
				Err(error) => Some(error.to_string()),
			};

			match string_result {
				Some(msg) => println!("{} -> {}", id.to_string_lossy(), msg),
				None => {},
			};
			
		}

	}

	fn validate_and_update_metadata_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<Option<String>, Box<dyn Error>> {
		
		let validation_result = Validate::validate_file(file, options)?;

		let result_message = match validation_result {
			Result::Valid(valid) => valid.result_string(options),
			Result::Invalid(invalid) => Self::update_metdata_file(file, options, invalid)?,
		};

		Ok(result_message)
	}

	fn update_metdata_file(file: &DirEntry, options: &ValidateOptions, mut result: Invalid) -> std::result::Result<Option<String>, Box<dyn Error>> {
		// TODO: Not sure how to allocate a mut reference inside the match statement and borrow it later
		let mut borrowable_metadata = MetaData::new(Uuid::new_v4());

		let metadata: &mut MetaData = match &mut result {
			Invalid::MissingMetadata => {
				borrowable_metadata.with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::MissingMetadataHistory { metadata } | Invalid::TimestampMismatch { metadata } => {
				metadata.with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::HashMismatch { metadata, file_hash } => {
				// actually we probably don't want to update on hash mismatch...because that means the file got corrupted?
				// metadata.with_file_hash(file_hash.clone())
				return Ok(Some("not updating metadata due to hash mismatch".into()));
			}
		};

		if !options.dry_run {
			MetaData::update(file, &metadata)?;
		}

		Ok(result.result_string(options))
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

impl Valid {

	fn result_string(&self, options: &ValidateOptions) -> Option<String> {
		match options.log_level {
			LogLevel::Verbose => Some(format!("✔ {}", self.to_string()).into()),
			_ => None, // Print nothing if not verbose
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

	fn result_string(&self, options: &ValidateOptions) -> Option<String> {
		match options.log_level {
			_ => Some(format!("updated because {}", self.to_string())),
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
