use crate::asset::{Assets};
use crate::meta::{ActualMetadata, ExpectedMetadata};

use color_print::cprintln;
use std::error::{Error};
use std::fs::{DirEntry};
use std::path::{PathBuf};
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
	Success,
	MetadataMissing,
	Error(Box<dyn Error>)
}

pub struct Validate {

}

pub struct Stats {
	total: usize,
	success: usize,
	missing: usize,
	errors: usize,
}

impl Validate {

	pub fn validate_assets(directory: &PathBuf, options: &ValidateOptions) -> Result<bool, Box<dyn Error>> {

		let assets = Assets::new(&directory)?;
		let mut stats = Stats::init(assets.file_map.len());

		cprintln!("<cyan>Validating files [0/{}] ...</cyan>\n", stats.total);


		for (_id, dir_entry) in assets.file_map.iter() {

			let result = Self::validate_file(dir_entry, options.contents);

			match result {
				ValidationResult::Success => {
					stats.success += 1;
					if matches!(options.log_level, LogLevel::Verbose) {
						cprintln!("<green>{:#?} ✓</green>", dir_entry.file_name())
					}
				},
				ValidationResult::Error(err) => {
					stats.errors += 1;
					match options.log_level {
						LogLevel::Default => cprintln!("<red>{:#?} X</red>", dir_entry.file_name()),
						LogLevel::Verbose => cprintln!("<red>{:#?} failed: {}</red>", dir_entry.file_name(), err),
					}
				}
				ValidationResult::MetadataMissing => {
					stats.missing += 1;
					if matches!(options.log_level, LogLevel::Verbose) {
						cprintln!("<yellow>{:#?} ?</yellow>", dir_entry.file_name())
					}
				},
			}

		}

		cprintln!("");
		cprintln!("<cyan>Validated [{}/{}] files ...</cyan>", stats.total, stats.total);
		cprintln!("<green>  Success {}</green>", stats.success);
		cprintln!("<yellow>  Missing {}</yellow>", stats.missing);
		cprintln!("<red>  Errors {}</red>", stats.errors);
		cprintln!("");

		Ok(true)

	}

	pub fn update_assets(directory: &PathBuf, options: &ValidateOptions) -> Result<bool, Box<dyn Error>> {

		let missing = Assets::new(&directory)?.missing();
		let mut stats = Stats::init(missing.len());

		cprintln!("<cyan>Updating files [0/{}]...</cyan>", stats.total);

		for (_id, dir_entry) in missing.iter() {

			let result = Self::update_file(dir_entry, options.dry_run);

			match result {
				Ok(expected) => {
					stats.success += 1;
					if matches!(options.log_level, LogLevel::Verbose) {
						cprintln!("<green>{:#?} ✓</green>", dir_entry.file_name());
						// TODO: Maybe have another log level where we don't print expected...
						cprintln!("{:#?}", expected)
					}
				},
				Err(err) => {
					stats.errors += 1;
					match options.log_level {
						LogLevel::Default => cprintln!("<red>{:#?} X</red>", dir_entry.file_name()),
						LogLevel::Verbose => cprintln!("<red>{:#?} failed: {}</red>", dir_entry.file_name(), err),
					}
				}
			}

		}

		cprintln!("");
		cprintln!("<cyan>Updated [{}/{}] files ...</cyan>", stats.total, stats.total);
		cprintln!("<green>  Success {}</green>", stats.success);
		cprintln!("<red>  Errors {}</red>", stats.errors);
		cprintln!("");

		Ok(true)

	}

	fn validate_file(dir_entry: &DirEntry, use_checksum: bool) -> ValidationResult {
		let result = Self::validate_file_wrapped(dir_entry, use_checksum);
		match result {
			Ok(result) => result,
			Err(err) => ValidationResult::Error(err),
		}
	}

	fn validate_file_wrapped(dir_entry: &DirEntry, use_checksum: bool) -> Result<ValidationResult, Box<dyn Error>> {

		let expected = match ExpectedMetadata::fetch(dir_entry)? {
			Some(expected) => expected,
			None => return Ok(ValidationResult::MetadataMissing)
		};

		let actual = ActualMetadata::fetch(dir_entry, use_checksum)?;

		if actual.last_modified_time != expected.last_modified_time {
			return Err("last_modified_time mismatch".into());
		}

		if actual.file_size != expected.file_size {
			return Err("file_size mismatch".into());
		}

		if use_checksum {
			let actual_sha256 = actual.sha256.ok_or("sha256 not calculated???")?;
			if actual_sha256 != expected.sha256 {
				return Err(format!("sha256 mismatch expected: {}, actual: {}", actual_sha256, expected.sha256).into());
			}
		}

		Ok(ValidationResult::Success)

	}

	fn update_file(dir_entry: &DirEntry, dry_run: bool) -> Result<ExpectedMetadata, Box<dyn Error>> {
		let actual = ActualMetadata::fetch(dir_entry, true)?;
		let expected = ActualMetadata::to_expected(actual)?;

		if !dry_run {
			ExpectedMetadata::write(dir_entry, &expected)?;
		}

		Ok(expected)
	}

}

impl Stats {
	pub fn init(total: usize) -> Self {
		Stats {
			total: total,
			success: 0,
			missing: 0,
			errors: 0,
		}
	}
}
