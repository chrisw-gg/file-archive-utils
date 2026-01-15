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
	pub detailed: bool,
	pub verbose: bool,
}

pub struct Validate {

}
/*
pub enum ValidateResult<'d> {
	Success,
	MissingMetadata   { file: &'d DirEntry },
	TimestampMismatch { file: &'d DirEntry, meta_data: MetaData, file_timestamp: DateTime<Utc> },
	HashMismatch      { file: &'d DirEntry, meta_data: MetaData, file_hash: String },
	Error { message: String },
}
*/

struct UpdateResult {

}

enum ValidationResult {
	Valid { metadata: MetaData },
	MissingMetadata,
	TimestampMismatch,
	HashMismatch { metadata: MetaData, file_hash: FileHash },
}

impl Validate {

/*
	pub fn print_results(results: &HashMap<PathBuf, Result>) {
		for (id, result) in results {
			let str: &str = match &result {
				Result::Success => "Success",
				Result::MissingMetadata { file } => "Missing Metadata",
				Result::TimestampMismatch { file, meta_data, file_timestamp } => {
					&format!("Timestamp Mismatch {} != {}", meta_data.last_modified_time, file_timestamp)
				}
				Result::HashMismatch { file, meta_data, file_hash } => {
					&format!("Hash Mismatch {} != {}", meta_data.sha256, file_hash)
				}
				Result::Error { message } => message,
			};
			println!("{} -> {}", id.display(), str);
		}
	}
*/
	pub fn validate_and_update_metadata(assets: &Assets, options: &ValidateOptions) {

		for (id, file) in assets.file_map.iter() {

			let result = Validate::validate_and_update_metadata_file(file, options);

			let string_result = match result {
				// TODO: Change return type of function, need to know if we updated a file or not...
				Ok(metadata) => {
					"TODO".to_string()
				}
				Err(error) => error.to_string()
			};

			println!("{} -> {}", id.to_string_lossy(), string_result);
		}

	}

	fn validate_and_update_metadata_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<MetaData, Box<dyn Error>> {
		
		let result = Validate::validate_file(file, options)?;

		let metadata = match result {
			ValidationResult::Valid { metadata } => metadata,
			ValidationResult::MissingMetadata | ValidationResult::TimestampMismatch => {

				let file_hash = Crypto::sha256(file)?;

				MetaData {
					id: Uuid::new_v4().into(),
					name: file.path().to_string_lossy().to_string(),
					file_hash: file_hash,
				}

			},
			ValidationResult::HashMismatch { metadata, file_hash } => {
				metadata
			}
		};
		
		Ok(metadata)

	}

	fn validate_file(file: &DirEntry, options: &ValidateOptions) -> std::result::Result<ValidationResult, Box<dyn Error>> {

		let last_modified_time = Directory::last_modified_time(file);

		let metadata = match meta::read_file_metadata(file) {
			Ok(metadata) => metadata,
			Err(metadata_error) => {
				match metadata_error {
					MetaDataError::NotFound => {
						options.verbose.then(|| println!("{} -> missing metadata", file.path().display()));
						return Ok(ValidationResult::MissingMetadata);
					}
					// Do not handle io or parse errors for now, will need to manually fix if this ever happens
					MetaDataError::ReadError { error } => {
						options.verbose.then(|| println!("{} -> read error:\n{}", file.path().display(), error));
						return Err("io error reading metadata file".into());
					}
					MetaDataError::ParseError { error } => {
						options.verbose.then(|| println!("{} -> parse error:\n{}", file.path().display(), error));
						return Err("parse error reading metadata file".into());
					}
				}
			}
		};

		// TODO: Check if metadata is later than file timestamp...

		if last_modified_time != metadata.file_hash.last_modified_time {
			options.verbose.then(|| println!("{} -> timestamp mismatch file.time({}) != metadata.time({})", file.path().display(), last_modified_time, metadata.file_hash.last_modified_time));
			return Ok(ValidationResult::TimestampMismatch);
		}

		if options.contents {

			let file_hash= Crypto::sha256(file)?;

			if file_hash.sha256 != metadata.file_hash.sha256 {
				options.verbose.then(|| println!("{} -> hash mismatch file.hash({:?}) != metadata.hash({:?})", file.path().display(), file_hash, metadata.file_hash));
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