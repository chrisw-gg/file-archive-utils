use indicatif::{ProgressBar};
use sha2::{Sha256, Digest};
use std::convert::TryInto;
use std::fs::{File};
use std::error::Error;
use std::io::{Read};

pub struct Crypto {

}

impl Crypto {

	pub fn stream_sha256(open_file: &mut File, progress_handler: &ProgressBar) -> Result<(String, usize), Box<dyn Error>> {
		const CHUNK_SIZE: usize = 1048576; // 1 MB

		let mut buffer = [0u8; CHUNK_SIZE];

		let mut hasher = Sha256::new();
		let mut total_bytes_read = 0;

		loop {

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