use crate::asset::{Assets};
use crate::crypto::{Crypto, FileHash};
use crate::directory::{Directory};
use crate::meta::{MetaData};

use color_print::cprintln;
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};
use uuid::{Uuid};

#[derive(Debug)]
pub struct ValidateOptions {
	pub contents: bool,
	pub dry_run: bool,
	pub log_level: LogLevel,
}

#[derive(Debug)]
pub enum LogLevel {
	Default,
	Verbose,
}

pub struct Validate {

}

enum Result {
	Valid(Valid),
	Invalid(Invalid),
}

enum Valid {
	TimestampMatches,
	HashAndTimestampMatches,
}

enum Invalid {
	MissingMetadata,
	MissingMetadataHistory { metadata: MetaData },
	FileModified { metadata: MetaData },
	HashMismatch { metadata: MetaData, file_hash: FileHash },
}

impl Validate {

	pub fn validate_and_update_metadata(assets: &Assets, options: &ValidateOptions) -> std::result::Result<bool, Box<dyn Error>> {

		let mut invalids: Vec<Invalid> = Vec::new();

		cprintln!("<cyan>Validating files...</cyan>");

		for (id, file) in assets.file_map.iter() {

			let val = Validate::validate_file(file)?;

			// val.print_line(id, options);

			match val {
				Result::Valid(valid) => valid.print_line(id, options),
				Result::Invalid(invalid) => {
					invalid.print_line(id, options);
					invalids.push(invalid);
				}
			};

		}

		cprintln!("<cyan>Creating changeset...</cyan>");

		let mut pending_changes: Vec<MetaData> = Vec::new();

		for invalid in invalids.iter() {
			
			let metadata = match invalid {
				Invalid::MissingMetadata => {
					MetaData::new(Uuid::new_v4()).with_file_hash(Crypto::sha256(file)?)
				},
				Invalid::MissingMetadataHistory { metadata } | Invalid::FileModified { metadata } => {
					metadata.with_file_hash(Crypto::sha256(file)?)
				},
				Invalid::HashMismatch { metadata, file_hash } => {
					metadata.with_file_hash(file_hash.clone()) // TODO: Do not deal with these ones!!! Create a third state -> Corrupted....
				}
			};

			pending_changes.push(metadata);

		}

		//if !options.dry_run {
			//	Self::update_metdata_file(file, options, invalid)?
			//}

		//

		Ok(true)

	}

	fn update_metdata_file(file: &DirEntry, _options: &ValidateOptions, result: &Invalid) -> std::result::Result<(), Box<dyn Error>> {

		let metadata = match result {
			Invalid::MissingMetadata => {
				MetaData::new(Uuid::new_v4()).with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::MissingMetadataHistory { metadata } | Invalid::FileModified { metadata } => {
				metadata.with_file_hash(Crypto::sha256(file)?)
			},
			Invalid::HashMismatch { metadata, file_hash } => {
				metadata.with_file_hash(file_hash.clone())
			}
		};

		// Don't update if there is a hash mismatch, this most likely means that the one of the files has been corrupted!
		match result {
			Invalid::HashMismatch { .. } => (),
			_ => MetaData::update(file, &metadata)?,
		}

		Ok(())

	}

	fn validate_file(file: &DirEntry) -> std::result::Result<Result, Box<dyn Error>> {
		let metadata = MetaData::read(file)?;

		let Some(metadata_file_hash) = metadata.last_file_hash() else {
			return Ok(Result::Invalid(Invalid::MissingMetadataHistory { metadata: metadata }));
		};

		if metadata_file_hash.last_modified_time != Directory::last_modified_time(file) {
			return Err("timestamp mismatch".into());
		}

		// TODO: if metadata_file_hash.file_size != file_size

		let file_hash= Crypto::sha256(file)?;

		if file_hash.sha256 != metadata_file_hash.sha256 {
			return Ok(Result::Invalid(Invalid::HashMismatch { metadata, file_hash }));
		}

		let result = Result::Valid(Valid::HashAndTimestampMatches);
		Ok(result)

	}

}

impl Result {

	fn print_line(&self, id: &PathBuf, options: &ValidateOptions) {

		match self {
			Result::Valid(valid) => valid.print_line(id, options),
			Result::Invalid(invalid) => invalid.print_line(id, options),
		}

	}

}

impl Valid {

	fn print_line(&self, id: &PathBuf, options: &ValidateOptions) {
		match options.log_level {
			LogLevel::Verbose => cprintln!("<green>{} -> {}</green>", id.to_string_lossy(), self.to_string()),
			_ => return // Print nothing if not verbose
		}
	}

	fn to_string(&self) -> &str {
		match self {
			Valid::TimestampMatches => "✓ timestamp",
			Valid::HashAndTimestampMatches => "✓ hash ✓ timestamp",
		}
	}

}

impl Invalid {

	fn print_line(&self, id: &PathBuf, _options: &ValidateOptions) {
		match self {
			Invalid::HashMismatch { .. } => cprintln!("<bright-red>{} -> {}<bright-red>", id.to_string_lossy(), self.to_string()),
			_ => cprintln!("<cyan>{} -> {}</cyan>", id.to_string_lossy(), self.to_string()),
		}
		
	}

	fn to_string(&self) -> &str {
		match self {
			Invalid::MissingMetadata => "missing metadata",
			Invalid::MissingMetadataHistory { .. } => "missing metadata history",
			Invalid::FileModified { .. } => "file modified",
			Invalid::HashMismatch { .. } => "hash mismatch",
		}
	}

}