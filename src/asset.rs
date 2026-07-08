use crate::directory::{Directory};
use std::collections::{HashMap};
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{Path, PathBuf};

pub struct Assets {
	pub file_map: HashMap<PathBuf, DirEntry>,
	pub meta_map: HashMap<PathBuf, DirEntry>,
}

impl Assets {
	
	pub fn new(directory: &Path) -> Result<Assets, Box<dyn Error>> {

		let files = Directory::read_files(directory)?;

		let mut assets = Assets {
			file_map: HashMap::new(),
			meta_map: HashMap::new(),
		};

		for file in files {

			if !file.file_type()?.is_file() {
				continue;
			}

			let mut path = file.path();
			let is_meta_file = file.path().extension().unwrap_or_default() == "meta";

			if is_meta_file {
				// id of an Asset is the file path (minus the .meta)
				path.set_extension("");
				assets.meta_map.insert(path, file);
			} else {
				assets.file_map.insert(path, file);
			}

		}

		Ok(assets)
		
	}

	pub fn missing(self) -> HashMap<PathBuf, DirEntry> {
		let mut missing: HashMap<PathBuf, DirEntry> = HashMap::new();

		for (path, dir_entry) in self.file_map {
			if !self.meta_map.contains_key(&path) {
				missing.insert(path, dir_entry);
			}
		}

		missing
	}

}