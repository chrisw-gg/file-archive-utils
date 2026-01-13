use chrono::{DateTime, Utc};
use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
	pub id: String,
	pub name: String,
	pub last_modified_time: DateTime<Utc>,
	pub sha256: String,
}

pub enum MetaDataError {
	NotFound,
	ReadError { error: String },
	ParseError { error: String },
}

pub fn read_file_metadata(file: &DirEntry) -> Result<MetaData, MetaDataError> {

	// File metadata is just stored as a file <filename>.meta on disk
	let mut path = file.path();
	path.add_extension(".meta");

	let file_contents = match fs::read_to_string(path) {
		Ok(contents) => contents,
		Err(error) => {
			let result = if error.kind() == std::io::ErrorKind::NotFound {
				MetaDataError::NotFound
			} else {
				MetaDataError::ReadError { error: error.to_string() }
			};
			return Err(result)
		}

	};

	let meta_data = match serde_saphyr::from_str(&file_contents) {
		Ok(meta_data) => meta_data,
		Err(error) => {
			let result = MetaDataError::ParseError { error: error.to_string() };
			return Err(result);
		}
	};

	Ok(meta_data)
}

pub fn write(meta_data: &MetaData, path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let yaml = serde_saphyr::to_string(&meta_data)?;
	fs::write(path, yaml)?;

	Ok(())
}
