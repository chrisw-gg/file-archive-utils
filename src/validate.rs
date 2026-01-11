#[path = "asset.rs"]
mod asset;

#[path = "crypto.rs"]
mod crypto;

#[path = "directory.rs"]
mod directory;

#[path = "meta.rs"]
mod meta;

use std::fs::{DirEntry};
use std::path::{PathBuf};

use crate::asset::{Assets};

pub struct Validate {

}

pub enum Result {
	Unknown,
	MissingFile,
	HashMismatch { actual: String, expected: String },
	Success
}

impl Validate {

	// TODO: Could multithread individual files and/or multiple files at a time
	pub fn validate(assets: &Assets) -> Vec<Result> {

		let mut results: Vec<Result> = Vec::new();

		for (id, entry) in assets.file_map.iter() {
			let result = Validate::validate_file(assets, &id, entry);
			results.push(result);
		}

		return results

	}

	pub fn print_results(results: Vec<Result>) {
		for result in results {
			let str = match result {
				Result::Unknown => "Unknown Error",
				Result::MissingFile => "Missing File",
				Result::HashMismatch { actual, expected } => "Hash Mismatch",
				Result::Success => "Success",
			};
			println!("{str}");
		}
	}

	fn validate_file(assets: &Assets, id: &PathBuf, entry: &DirEntry) -> Result {
		
		let Some(meta_entry) = assets.meta_map.get(id) else {
			return Result::MissingFile
		};

		let Ok(meta_data) = meta::read(&meta_entry.path()) else {
			// TODO: Better error from meta read
			return Result::Unknown
		};

		let Ok(actual_hash) = crypto::sha256_file(entry.path()) else {
			// TODO: What errors does this return?
			return Result::Unknown
		};

		if actual_hash != meta_data.sha256 {
			return Result::HashMismatch { actual: actual_hash, expected: meta_data.sha256 }
		}

		return Result::Success;

	}

}