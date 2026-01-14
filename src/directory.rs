use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{Path};

pub fn read_files(directory: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
	let path = Path::new(&directory);

	let iter = &mut DirectoryIter {
		files: Vec::new()
	};

	iter.walk_directory(path)?;

	let files = std::mem::take(&mut iter.files);

	Ok(files)
}

struct DirectoryIter {
	files: Vec<DirEntry>,
}

impl DirectoryIter {

	fn walk_directory(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {

		for result in fs::read_dir(path)? {

			let entry = result?;

			if entry.file_type()?.is_dir() {
				self.walk_directory(&entry.path())?;
			}

			self.files.push(entry);

		}

		Ok(())
	}

}