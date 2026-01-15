#![allow(warnings)] // TODO: Temporary while refactoring

mod asset;
mod crypto;
mod directory;
mod meta;
mod validate;

use std::env;
use std::error::{Error};

use asset::{Assets};
use validate::{LogLevel, Validate, ValidateOptions};

fn main() {
	_ = run();
}

fn run() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	let options = ValidateOptions {
		timestamps: true, // always compare timestamps (contents will just potentially override this)
		contents: args.iter().find(|a| a.as_str() == "--contents").is_some(),
		dry_run: args.iter().find(|a| a.as_str() == "--dry-run").is_some(),
		log_level: if args.iter().find(|a| a.as_str() == "--verbose").is_some() {
			LogLevel::Verbose
		} else if args.iter().find(|a| a.as_str() == "--minimal").is_some() {
			LogLevel::Minimal
		} else {
			LogLevel::Default
		},
	};

	let assets = Assets::new().unwrap();

	Validate::validate_and_update_metadata(&assets, &options);
	
	Ok(())

}