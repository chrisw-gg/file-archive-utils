use crate::crypto::{self};
use crate::asset::{Assets};
use crate::meta::{self, MetaData, MetaDataError};

use chrono::{DateTime, Utc};
use std::collections::{HashMap};
use std::fs::{DirEntry};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ValidateOptions {
	pub timestamps: bool,
	pub contents: bool,
	pub dry_run: bool,
	pub detailed: bool,
}

pub struct Validate {

}

pub enum Result<'d> {
	Success,
	MissingMetadata   { file: &'d DirEntry },
	TimestampMismatch { file: &'d DirEntry, meta_data: MetaData, file_timestamp: DateTime<Utc> },
	HashMismatch      { file: &'d DirEntry, meta_data: MetaData, file_hash: String },
	Error { message: String },
}

pub enum HashResult {
	Success { last_modified_time: DateTime<Utc>, sha256: String },
	FileModified { before: DateTime<Utc>, after: DateTime<Utc> },
	Error { message: String },
}

impl Validate {

	// TODO: Could multithread individual files and/or multiple files at a time
	pub fn validate<'d>(assets: &'d Assets, options: &ValidateOptions) -> HashMap<PathBuf, Result<'d>> {

		let mut result_map: HashMap<PathBuf, Result<'d>> = HashMap::new();

		for (id, file) in assets.file_map.iter() {
			let validation_result = Validate::validate_file(file, options);
			result_map.insert(id.to_path_buf(), validation_result);

		}

		return result_map

	}

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

	fn validate_file<'d>(file: &'d DirEntry, options: &ValidateOptions) -> Result<'d> {

		let meta_data = match meta::read_file_metadata(file) {
			Ok(meta_data) => meta_data,
			Err(meta_data_error) => {
				let result = match meta_data_error {
					MetaDataError::NotFound => Result::MissingMetadata { file },
					MetaDataError::ReadError { error } => Result::Error { message: error },
					MetaDataError::ParseError { error } => Result::Error { message: error },
				};
				return result;
			}
		};

		let last_modified_time = Self::last_modified_time(file);
		if last_modified_time != meta_data.last_modified_time {
			return Result::TimestampMismatch { file: file, meta_data: meta_data, file_timestamp: last_modified_time }
		}

		if options.contents {
			
			let (last_modified, file_hash) = match Self::sha256(file) {
				HashResult::Success { last_modified_time, sha256 } => (last_modified_time, sha256),
				HashResult::FileModified { before, after } => return Result::Error { message: "File Modified".to_string() }, // TODO
				HashResult::Error { message } => return Result::Error { message: message },
			};

			if file_hash != meta_data.sha256 {
				return Result::HashMismatch { file, meta_data, file_hash }
			}

		}

		return Result::Success;

	}

	pub fn sha256(file: &DirEntry) -> HashResult {
		let modified_before = Self::last_modified_time(file);

		let file_hash = match crypto::sha256_file(file.path()) {
			Ok(hash) => hash,
			Err(error) => return HashResult::Error { message: error.to_string() },
		};

		let modified_after = Self::last_modified_time(file);

		// Attempt to ensure consistency in cases of file changing while hashing it
		// Compare the last modified time from before and after the hash
		// TODO: This may not actually work if the modified time is just stored in the direntry...
		if modified_before != modified_after {
			return HashResult::FileModified { before: modified_before, after: modified_after };
		}

		return HashResult::Success { last_modified_time: modified_before, sha256: file_hash }
	}

	// Panics if can't get last_modified_time of file
	fn last_modified_time(file: &DirEntry) -> DateTime<Utc> {

		let meta_data = file.metadata().unwrap();
		let modified = meta_data.modified().unwrap();

		let last_modified_time: DateTime<Utc> = modified.into();

		return last_modified_time;
	}

}