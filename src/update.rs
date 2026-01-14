use crate::meta::{self, MetaData};
use crate::validate::{self as validation, Validate, ValidateOptions};

use std::collections::{HashMap};
use std::error::{Error};
use std::path::PathBuf;


pub struct Update {

}

enum UpdateResult<'d> {
	UpToDate,
	Modified { meta_data: &'d MetaData },
	Error { message: String },
}

impl Update {

	pub fn update_meta_files(validation_results: &HashMap<PathBuf, validation::Result>, options: &ValidateOptions) {

		// File + optional hash (from validation)
		// let files_need_updating: Vec<FileToUpdate> = Vec::new();

		for (path, result) in validation_results.iter() {

			fn success_handler() {

			}

			fn write_meta_data(path: &PathBuf) {

			}

			// TODO: dry_run
			Self::update_meta_file(path, result);

		}

	}

	fn update_meta_file(path: &PathBuf, result: &validation::Result) {

		let meta_data = match result {
			validation::Result::Success => {
				UpdateResult::UpToDate
			}
			validation::Result::MissingMetadata { file } => {
				/*
				let result = match Validate::sha256(path) {
					validation::HashResult::Success { last_modified_time, sha256 } => todo!(),
					validation::HashResult::FileModified { before, after } => todo!(),
					validation::HashResult::Error { message } => todo!(),
				};
				*/
				UpdateResult::UpToDate
			}
			validation::Result::TimestampMismatch { file, meta_data, file_timestamp } => {

				match Validate::sha256(file) {
					validation::HashResult::Success { last_modified_time, sha256 } => {

					}
					validation::HashResult::FileModified { before, after } => todo!(),
					validation::HashResult::Error { message } => todo!(),
				}

				UpdateResult::UpToDate
			}
			validation::Result::HashMismatch { file, meta_data, file_hash } => {
				// Hash mismatch is a special case where we can use the existing hash we computed to speed things up
				// Append new hash and timestamp to metadata
				// meta_data.sha256 = file_hash;
				// Result::Modified { meta_data }
				UpdateResult::UpToDate
			}
			validation::Result::Error { message } => {
				println!("{message}");
				UpdateResult::UpToDate
			}
		};

	}

}