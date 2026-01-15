use crate::asset::{Assets};
use crate::crypto::{Crypto, FileHash, HashResult};
use crate::directory::{Directory};
use crate::meta::{self, MetaData, MetaDataError};

use chrono::{DateTime, Utc};
use std::collections::{HashMap};
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::{Uuid};


pub struct ValidateOptions {
	pub timestamps: bool,
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

struct UpdateResult {

}

enum ValidationResult {
	Valid { metadata: MetaData },
	MissingMetadata,
	TimestampMismatch,
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
		
		let result = Validate::validate_file(file, options)?;

		let update_because = match &result {
			ValidationResult::Valid { .. } => "no update",
			ValidationResult::MissingMetadata => "updated because missing metadata",
			ValidationResult::TimestampMismatch => "updated because timestamp mismatch",
			ValidationResult::HashMismatch { .. } => "updated because hash mismatch",
		};

		let metadata = match result {
			ValidationResult::Valid { metadata } => {
				
				return Ok(match options.log_level {
					LogLevel::Verbose => Some("good".into()),
					_ => None,
				});

			}
			ValidationResult::MissingMetadata | ValidationResult::TimestampMismatch => { // NO... WRONG timestamp has metadata file already

				let file_hash = Crypto::sha256(file)?;

				MetaData {
					id: Uuid::new_v4().into(),
					name: file.path().to_string_lossy().to_string(),
					file_hash: file_hash,
				}

			},
			ValidationResult::HashMismatch { metadata, file_hash } => {
				// metadata.file_hash = file_hash;
				metadata
			}
		};

		if !options.dry_run {
			MetaData::update(file, &metadata)?;
		}

		let msg = match options.log_level {
			LogLevel::Verbose => format!("{} -> {:?}", update_because, metadata),
			_ => format!("{}", update_because),
		};

		Ok(Some(msg))

	}

	fn validate_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<ValidationResult, Box<dyn Error>> {

		let last_modified_time = Directory::last_modified_time(file);

		let metadata = match MetaData::read(file) {
			Ok(metadata) => metadata,
			Err(metadata_error) => {
				match metadata_error {
					MetaDataError::NotFound => {
						return Ok(ValidationResult::MissingMetadata);
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

		// TODO: Check if metadata is later than file timestamp...

		if last_modified_time != metadata.file_hash.last_modified_time {
			// options.verbose.then(|| println!("{} -> timestamp mismatch file.time({}) != metadata.time({})", file.path().display(), last_modified_time, metadata.file_hash.last_modified_time));
			return Ok(ValidationResult::TimestampMismatch);
		}

		if options.contents {

			let file_hash= Crypto::sha256(file)?;

			if file_hash.sha256 != metadata.file_hash.sha256 {
				// options.verbose.then(|| println!("{} -> hash mismatch file.hash({:?}) != metadata.hash({:?})", file.path().display(), file_hash, metadata.file_hash));
				return Ok(ValidationResult::HashMismatch {
					metadata: metadata,
					file_hash: file_hash
				});
			}

		}

		let result = ValidationResult::Valid { metadata: metadata };
		Ok(result)

	}

}