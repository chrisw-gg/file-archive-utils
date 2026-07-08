use chrono::{DateTime, Utc};
use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
	pub id: String,
	pub file_name: String,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: String,
}

impl MetaData {

	pub fn read(file: &DirEntry) -> Result<MetaData, Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file);
		let file_contents = fs::read_to_string(&path)?;
		let meta_data = serde_saphyr::from_str(&file_contents)?;
		Ok(meta_data)
	}

	pub fn update(file: &DirEntry, meta_data: &MetaData) -> Result<(), Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file);

		let yaml = serde_saphyr::to_string(&meta_data)?;
		fs::write(path, yaml)?;

		Ok(())
	}

	fn path_for_metadata_file(file: &DirEntry) -> PathBuf {
		file.path().with_added_extension("meta")
	}

}