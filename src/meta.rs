use crate::crypto::Crypto;
use crate::validate::ValidateOptions;

use chrono::{DateTime, Utc};
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Serialize, Deserialize};
use std::error::{Error};
use std::fs::{self, DirEntry, File};
use std::path::{PathBuf};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpectedMetadata {
	pub id: String,
	pub file_name: String,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActualMetadata {
	pub file_name: String,
	pub last_modified_time: DateTime<Utc>,
	pub file_size: u64,
	pub sha256: Option<String>,
}

impl ExpectedMetadata {

	pub fn fetch(file: &DirEntry, options: &ValidateOptions) -> Result<Option<ExpectedMetadata>, Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file, options)?;

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

	pub fn write(file: &DirEntry, meta_data: &ExpectedMetadata, options: &ValidateOptions) -> Result<(), Box<dyn Error>> {
		let path = Self::path_for_metadata_file(file, options)?;
		let parent = path.parent().ok_or("No parent")?;

		if !fs::exists(parent)? {
			fs::create_dir(parent)?;
		}

		let yaml = serde_saphyr::to_string(&meta_data)?;
		fs::write(path, yaml)?;

		Ok(())
	}

	// test/foo.jpg -> test/foo.jpg.meta (inline)
	// test/foo.jpg -> test/.metadata/foo.jpg.meta
	fn path_for_metadata_file(file: &DirEntry, options: &ValidateOptions) -> Result<PathBuf, Box<dyn Error>> {
		if options.inline {
			return Ok(file.path().with_added_extension("meta"))
		}

		let file_path = file.path();
		let parent = file_path.parent().ok_or("No parent")?;
		let file_name = file_path.file_name().ok_or("No filename")?;
		let metadata_path = parent.join(".metadata").join(file_name).with_added_extension("meta");
		Ok(metadata_path)
	}

}

impl ActualMetadata {

	pub fn fetch(dir_entry: &DirEntry, include_checksum: bool) -> Result<ActualMetadata, Box<dyn Error>> {
		let mut file = File::open(dir_entry.path())?;

		// TODO: Failing to acquire lock frequently...just windows things?
		// file.try_lock_shared()?;
		
		let mut actual_metadata = ActualMetadata {
			file_name: dir_entry.file_name().to_string_lossy().into(),
			last_modified_time: dir_entry.metadata()?.modified()?.into(),
			file_size: dir_entry.metadata()?.len(),
			sha256: None,
		};

		if include_checksum {
			let pb: ProgressBar = ProgressBar::new(actual_metadata.file_size);
			pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
			.progress_chars("#>-"));
			pb.tick();

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