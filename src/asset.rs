use crate::directory::{Directory};
use std::collections::{HashMap};
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};

pub struct Assets {
	pub file_map: HashMap<PathBuf, DirEntry>,
	pub meta_map: HashMap<PathBuf, DirEntry>,
}

impl Assets {
	
	pub fn new() -> Result<Assets, Box<dyn Error>> {

		let files = Directory::read_files("/workdir/foo")?;

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

}