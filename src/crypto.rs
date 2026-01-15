use chrono::{DateTime, Utc};
use std::fs::{File, DirEntry};
use std::error::Error;
use std::path::{PathBuf};
use sha2::{Sha256, Digest};
use std::io::{Read};

pub struct Crypto {

}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileHash {
	pub path: PathBuf,
	pub last_modified_time: DateTime<Utc>,
	pub sha256: String,
}

impl Crypto {

	pub fn sha256(file: &DirEntry) -> Result<FileHash, Box<dyn Error>> {
		let path = file.path();
		let mut open_file = File::open(&path)?;

		open_file.lock()?;

		let file_modified:  DateTime<Utc> = std::fs::metadata(&path)?.modified()?.into();
		let file_hash = Self::stream_sha256(&mut open_file)?;

		open_file.unlock()?;

		let result = FileHash {
			path: path,
			last_modified_time: file_modified,
			sha256: file_hash
		};

		Ok(result)
	}

	fn stream_sha256(open_file: &mut File) -> Result<String, Box<dyn Error>> {
		const CHUNK_SIZE: usize = 1024;

		let mut hasher = Sha256::new();

		loop {

			let mut buffer = [0u8; CHUNK_SIZE];

			let bytes_read = open_file.read(&mut buffer)?;

			if bytes_read == 0 {
				break;
			}

			hasher.update(&buffer[..bytes_read]);

		}

		let hash = hasher.finalize();
		let base64 = format!("{:x}", hash);

		Ok(base64)
	}

}