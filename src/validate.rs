#[path = "asset.rs"]
mod asset;

#[path = "crypto.rs"]
mod crypto;

#[path = "directory.rs"]
mod directory;

#[path = "meta.rs"]
mod meta;

use crate::asset::{Assets};

use chrono::{DateTime, Utc};
use std::collections::{HashMap};
use std::fs::{DirEntry};
use std::path::{PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ValidateOptions {
	pub timestamps: bool,
	pub contents: bool,
}

pub struct Validate {

}

pub enum Result {
	Error { message: String },
	HashMismatch { actual: String, expected: String },
	MissingFile,
	Success,
	TimestampMismatch { before: DateTime<Utc>, after: DateTime<Utc> },
	Unknown,
}

pub enum HashResult {
	Success { last_modified_time: DateTime<Utc>, sha256: String },
	FileModified { before: DateTime<Utc>, after: DateTime<Utc> },
	Error { message: String },
}

impl Validate {

	// TODO: Could multithread individual files and/or multiple files at a time
	pub fn validate(assets: &Assets, options: &ValidateOptions) -> HashMap<PathBuf, Result> {

		let mut result_map: HashMap<PathBuf, Result> = HashMap::new();

		for (id, file) in assets.file_map.iter() {
			let validation_result = Validate::validate_file(file, options);
			result_map.insert(id.to_path_buf(), validation_result);

		}

		return result_map

	}

	pub fn print_results(results: HashMap<PathBuf, Result>) {
		for (id, result) in results {
			let str: &str = match &result {
				Result::Error { message } => message,
				Result::HashMismatch { actual, expected } => &format!("Hash Mismatch {actual} != {expected}"),
				Result::MissingFile => "Missing File",
				Result::Success => "Success",
				Result::TimestampMismatch { before, after } => &format!("Timestamp Mismatch {before} != {after}"),
				Result::Unknown => "Unknown Error",
			};
			println!("{} -> {}", id.display(), str);
		}
	}

	fn validate_file(file: &DirEntry, options: &ValidateOptions) -> Result {

		let meta_data = match meta::read_file_metadata(file) {
			Ok(meta_data) => meta_data,
			Err(meta_data_error) => {
				let result = match meta_data_error {
					meta::MetaDataError::NotFound => Result::MissingFile,
					meta::MetaDataError::ReadError { error } => Result::Error { message: error },
					meta::MetaDataError::ParseError { error } => Result::Error { message: error },
				};
				return result;
			}
		};

		if options.timestamps {
			if Self::last_modified_time(file) == meta_data.last_modified_time {
				return Result::Success
			}
		}

		if options.contents {
			
			let (last_modified, file_hash) = match Self::sha256(file) {
				HashResult::Success { last_modified_time, sha256 } => (last_modified_time, sha256),
				HashResult::FileModified { before, after } => return Result::Error { message: "File Modified".to_string() }, // TODO
				HashResult::Error { message } => return Result::Error { message: message },
			};

			if file_hash != meta_data.sha256 {
				return Result::HashMismatch { actual: file_hash, expected: meta_data.sha256 }
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