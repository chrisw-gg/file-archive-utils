use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use std::convert::TryInto;
use std::fs::{File, DirEntry};
use std::error::Error;
use sha2::{Sha256, Digest};
use std::io::{Read};

pub struct Crypto {

}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileHash {
	pub file_name: String,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: String,
}

impl Crypto {

	pub fn sha256(file: &DirEntry) -> Result<FileHash, Box<dyn Error>> {
		let path = file.path();
		let mut open_file = File::open(&path)?;

		open_file.lock()?;

		let metadata = std::fs::metadata(&path)?;
		
		let file_modified:  DateTime<Utc> =metadata.modified()?.into();
		let file_size = metadata.len();

		let pb: ProgressBar = ProgressBar::new(file_size);
		pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
		.progress_chars("#>-"));

		let (file_hash, bytes_read) = Self::stream_sha256(&mut open_file, &pb)?;

		pb.finish_and_clear();

		open_file.unlock()?;

		let Some(file_name) = path.file_name() else {
			return Err("Could not get file name".into());
		};

		if file_size != bytes_read.try_into()? {
			return Err("File size modified".into());
		}

		let result = FileHash {
			file_name: file_name.to_string_lossy().to_string(),
			last_modified_time: file_modified,
			file_size: file_size,
			sha256: file_hash
		};

		Ok(result)
	}

	fn stream_sha256(open_file: &mut File, progress_handler: &ProgressBar) -> Result<(String, usize), Box<dyn Error>> {
		const CHUNK_SIZE: usize = 1024;

		let mut hasher = Sha256::new();
		let mut total_bytes_read = 0;

		loop {

			let mut buffer = [0u8; CHUNK_SIZE];

			let bytes_read = open_file.read(&mut buffer)?;
			total_bytes_read += bytes_read;

			if bytes_read == 0 {
				break;
			}

			progress_handler.inc(bytes_read.try_into()?);

			hasher.update(&buffer[..bytes_read]);

		}

		let hash = hasher.finalize();
		let base64 = format!("{:x}", hash);

		Ok((base64, total_bytes_read))
	}

}