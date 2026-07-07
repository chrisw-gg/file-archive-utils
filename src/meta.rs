use crate::crypto::{FileHash};

use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};
use uuid::{Uuid};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
	id: String,
	history: Vec<FileHash>,
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

	pub fn previous_file_hash(&self) -> Option<&FileHash> {
		let mut rev = self.history.iter().rev();
		let (_first, second) = (rev.next(), rev.next());
		second
	}

	pub fn with_file_hash(&self, file_hash: FileHash) -> Self {
		let mut clone = self.clone();
		clone.history.push(file_hash);
		clone
	}

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