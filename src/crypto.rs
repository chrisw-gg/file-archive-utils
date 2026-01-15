use crate::directory::{Directory};

use chrono::{DateTime, Utc};
use std::fs::{File, DirEntry};
use std::error::Error;
use std::path::{PathBuf};
use sha2::{Sha256, Digest};
use std::io::{BufRead, BufReader};

pub struct Crypto {

}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileHash {
	pub path: PathBuf,
	pub last_modified_time: DateTime<Utc>,
	pub sha256: String,
}

pub enum HashResult {
	Success { last_modified_time: DateTime<Utc>, sha256: String },
	FileModified { before: DateTime<Utc>, after: DateTime<Utc> },
	Error { message: String },
}

impl Crypto {

	pub fn sha256(file: &DirEntry) -> Result<FileHash, Box<dyn Error>> {
		// println!("Hashing file: {}", file.path().display());
		match Self::sha256_file(file) {
			HashResult::Success { last_modified_time, sha256 } => {
				let hash = FileHash{
					path: file.path(),
					last_modified_time: last_modified_time,
					sha256: sha256,
				};
				Ok(hash)
			}
			HashResult::FileModified { before, after } => {
				let message = format!("File modified while hashing before: {} after: {}", before, after);
				Err(message.into())
			}
			HashResult::Error { message } => Err(message.into())
		}
	}

	// Attempt to ensure consistency in cases of file changing while hashing it
	// Compare the last modified time from before and after the hash
	// TODO: This may not actually work if the modified time is just stored in the direntry...
	fn sha256_file(file: &DirEntry) -> HashResult {
		let modified_before = Directory::last_modified_time(file);

		let file_hash = match Self::stream_sha256(file.path()) {
			Ok(hash) => hash,
			Err(error) => return HashResult::Error { message: error.to_string() },
		};

		let modified_after = Directory::last_modified_time(file);

		if modified_before != modified_after {
			return HashResult::FileModified { before: modified_before, after: modified_after };
		}

		return HashResult::Success { last_modified_time: modified_before, sha256: file_hash }
	}

	// TODO: Does not match linux sha256sum...try reading file as binary in chunks
	fn stream_sha256(path: PathBuf) -> Result<String, Box<dyn Error>> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		let mut hasher = Sha256::new();

		for line_result in reader.lines() {
			let l = line_result?;
			println!("{}", &l);
			hasher.update(&l);
		}

		let hash = hasher.finalize();
		let base64 = format!("{:X}", hash);

		Ok(base64)
	}

}