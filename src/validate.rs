use crate::asset::{Assets};
use crate::meta::{ActualMetadata, ExpectedMetadata};

use color_print::cprintln;
use std::error::{Error};
use std::fs::{DirEntry};
use std::result::Result;

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

pub enum ValidationResult {
	Good,
	MetadataMissing,
	Error(Box<dyn Error>)
}

pub struct Validate {

}

impl Validate {

	pub fn validate_and_update_metadata(assets: &Assets, options: &ValidateOptions) -> Result<bool, Box<dyn Error>> {

		cprintln!("<cyan>Validating files...</cyan>");

		for (_id, dir_entry) in assets.file_map.iter() {

			let result = Self::validate_file(dir_entry, options);

			match result {
				ValidationResult::Good => {
					if matches!(options.log_level, LogLevel::Verbose) {
						cprintln!("<green>{:#?} ✓</green>", dir_entry.file_name())
					}
				},
				ValidationResult::Error(err) => {
					match options.log_level {
						LogLevel::Default => cprintln!("<red>{:#?} X</red>", dir_entry.file_name()),
						LogLevel::Verbose => cprintln!("<red>{:#?} failed: {}</red>", dir_entry.file_name(), err),
					}
				}
				ValidationResult::MetadataMissing => {
					let actual = ActualMetadata::fetch(dir_entry, true)?;
					let expected = ActualMetadata::to_expected(actual)?;

					if !options.dry_run {
						ExpectedMetadata::write(dir_entry, &expected)?;
					}

					if matches!(options.log_level, LogLevel::Verbose) {
						cprintln!("{:#?}", expected)
					}
				},
			}

		}

		// List statistics, good, bad, missing...
		cprintln!("<cyan>Validated [0/x] files...</cyan>");

		Ok(true)

	}

	fn validate_file(dir_entry: &DirEntry, options: &ValidateOptions) -> ValidationResult {
		let result = Self::validate_file_wrapped(dir_entry, options);
		match result {
			Ok(result) => result,
			Err(err) => ValidationResult::Error(err),
		}
	}

	fn validate_file_wrapped(dir_entry: &DirEntry, options: &ValidateOptions) -> Result<ValidationResult, Box<dyn Error>> {

		let expected = match ExpectedMetadata::fetch(dir_entry)? {
			Some(expected) => expected,
			None => return Ok(ValidationResult::MetadataMissing)
		};

		let actual = ActualMetadata::fetch(dir_entry, options.contents)?;

		if actual.last_modified_time != expected.last_modified_time {
			return Err("last_modified_time mismatch".into());
		}

		if actual.file_size != expected.file_size {
			return Err("file_size mismatch".into());
		}

		if options.contents {
			let actual_sha256 = actual.sha256.ok_or("sha256 not calculated???")?;
			if actual_sha256 != expected.sha256 {
				return Err(format!("sha256 mismatch expected: {}, actual: {}", actual_sha256, expected.sha256).into());
			}
		}

		// TODO: Return "✓ hash ✓ timestamp" maybe
		Ok(ValidationResult::Good)

	}

}