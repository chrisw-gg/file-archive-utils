use crate::crypto::{FileHash};

use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use uuid::{Uuid};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
	id: String,
	history: Vec<FileHash>,
}

pub enum MetaDataError {
	NotFound,
	ReadError { error: String },
	ParseError { error: String },
}

impl MetaData {

	pub fn new(id: Uuid) -> MetaData {
		MetaData {
			id: id.into(),
			history: Vec::new(),
		}
	}

	pub fn last_file_hash(&self) -> Option<&FileHash> {
		self.history.last()
	}

	pub fn with_file_hash(&mut self, file_hash: FileHash) -> &mut Self {
		self.history.push(file_hash);
		self
	}

	pub fn read(file: &DirEntry) -> Result<MetaData, MetaDataError> {
		let path = Self::path_for_metadata_file(file);

		let file_contents = match fs::read_to_string(&path) {
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