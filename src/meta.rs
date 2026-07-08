use crate::crypto::Crypto;

use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Serialize, Deserialize};
use std::error::{Error};
use std::ffi::OsString;
use std::fs::{self, DirEntry, File};
use std::path::{PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpectedMetadata {
	pub id: String,
	pub file_name: OsString,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActualMetadata {
	pub file_name: OsString,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: Option<String>,
}

impl ExpectedMetadata {

	pub fn fetch(file: &DirEntry) -> Result<Option<ExpectedMetadata>, Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file);

		let file_contents = match fs::read_to_string(&path) {
			Ok(contents) => contents,
			Err(error) => {
				if error.kind() == std::io::ErrorKind::NotFound {
					return Ok(None)
				} else {
					return Err(error.into());
				};
			}
		};

		let meta_data = serde_saphyr::from_str(&file_contents)?;
		Ok(Some(meta_data))
	}

	pub fn write(file: &DirEntry, meta_data: &ExpectedMetadata) -> Result<(), Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file);

		let yaml = serde_saphyr::to_string(&meta_data)?;
		fs::write(path, yaml)?;

		Ok(())
	}

	fn path_for_metadata_file(file: &DirEntry) -> PathBuf {
		file.path().with_added_extension("meta")
	}

}

impl ActualMetadata {

	pub fn fetch(dir_entry: &DirEntry, include_checksum: bool) -> Result<ActualMetadata, Box<dyn Error>> {
		let mut file = File::open(dir_entry.path())?;

		file.lock()?;
		
		let mut actual_metadata = ActualMetadata {
			file_name: dir_entry.file_name(),
			last_modified_time: dir_entry.metadata()?.modified()?.into(),
			file_size: dir_entry.metadata()?.len(),
			sha256: None,
		};

		if include_checksum {
			let pb: ProgressBar = ProgressBar::new(actual_metadata.file_size);
			pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
			.progress_chars("#>-"));

			let (sha256, _bytes_read) = Crypto::stream_sha256(&mut file, &pb)?;
			// TODO: Check bytes_read?
			actual_metadata.sha256 = Some(sha256);
		}

		Ok(actual_metadata)

	}

	pub fn to_expected(actual: ActualMetadata) -> Result<ExpectedMetadata, Box<dyn Error>> {
		let expected = ExpectedMetadata {
			id: Uuid::new_v4().into(),
			file_name: actual.file_name,
			last_modified_time: actual.last_modified_time,
			file_size: actual.file_size,
			sha256: actual.sha256.ok_or("sha256 not calculated???")?,
		};
		Ok(expected)
	}

}