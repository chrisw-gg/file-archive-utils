use crate::directory::{self};
use std::collections::{HashMap};
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};

pub struct Assets {
	pub file_map: HashMap<PathBuf, DirEntry>,
	pub meta_map: HashMap<PathBuf, DirEntry>,
	pub hash_map: HashMap<PathBuf, String>,
}

impl Assets {
	
	pub fn new() -> Result<Assets, Box<dyn Error>> {

		let files = directory::read_files("/workdir/foo")?;

		let mut assets = Assets {
			file_map: HashMap::new(),
			meta_map: HashMap::new(),
			hash_map: HashMap::new(),
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

impl Assets {
	
	pub fn missing_meta_files(&self) -> Vec<&DirEntry> {
		let mut files: Vec<&DirEntry> = Vec::new();
		for (id, file) in self.file_map.iter() {
			if !self.meta_map.contains_key(id) {
				files.push(file);
			}
		}
		return files;
	}

}

impl Assets {

	pub fn print_report(&self) {
		println!("\n\nReport\n\n");

		let missing_meta_files = self.missing_meta_files();

		println!("Missing meta files ({}):\n", missing_meta_files.len());
		for file in missing_meta_files {
			println!("{}", file.file_name().display())
		}

		println!("\n\nEnd of Report\n\n");
	}

	#[allow(dead_code)]
	pub fn debug_print(&self) {
		println!("file_map: {:#?}", self.file_map);
		println!("meta_map: {:#?}", self.meta_map);
		println!("hash_map: {:#?}", self.hash_map);
	}

}