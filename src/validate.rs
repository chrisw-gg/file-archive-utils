use crate::asset::{Assets};
use crate::crypto::{Crypto};
use crate::directory::{Directory};
use crate::meta::{MetaData};

use color_print::cprintln;
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};
// use uuid::{Uuid};

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
}

enum Valid {
	TimestampMatches,
	HashAndTimestampMatches,
}

impl Validate {

	pub fn validate_and_update_metadata(assets: &Assets, options: &ValidateOptions) -> std::result::Result<bool, Box<dyn Error>> {

		cprintln!("<cyan>Validating files...</cyan>");

		for (id, file) in assets.file_map.iter() {

			let val = Validate::validate_file(file)?;

			// val.print_line(id, options);

			match val {
				Result::Valid(valid) => valid.print_line(id, options),
			};

		}

		//if !options.dry_run {
			//	Self::update_metdata_file(file, options, invalid)?
			//}

		//

		Ok(true)

	}

	fn validate_file(file: &DirEntry) -> std::result::Result<Result, Box<dyn Error>> {
		// TODO: Return optional metadata... because it may not exist and that is not an error...
		let metadata = MetaData::read(file)?;

		if metadata.last_modified_time != Directory::last_modified_time(file) {
			return Err("timestamp mismatch".into());
		}

		// TODO: if metadata_file_hash.file_size != file_size

		// TODO: Don't necessarily want to hash the file for quick checks...pass in options
		let file_hash= Crypto::sha256(file)?;

		if metadata.sha256 != file_hash.sha256 {
			return Err(format!("sha256 mismatch expected: {}, actual: {}", metadata.sha256, file_hash.sha256).into());
		}

		let result = Result::Valid(Valid::HashAndTimestampMatches);
		Ok(result)

	}

}

impl Result {

	fn print_line(&self, id: &PathBuf, options: &ValidateOptions) {

		match self {
			Result::Valid(valid) => valid.print_line(id, options),
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