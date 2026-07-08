use std::error::{Error};
use std::fs::{self, DirEntry};
use std::path::{Path};

pub struct Directory {

}

struct DirectoryIter {
	files: Vec<DirEntry>,
}

impl Directory {

	pub fn read_files(directory: &Path) -> Result<Vec<DirEntry>, Box<dyn Error>> {

		let iter = &mut DirectoryIter {
			files: Vec::new()
		};

		iter.walk_directory(directory)?;

		let files = std::mem::take(&mut iter.files);

		Ok(files)
	}

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
