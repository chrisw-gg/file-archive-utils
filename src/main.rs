// #![allow(warnings)] // TODO: Temporary while refactoring

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
	run();
}

fn run() -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	let options = options();

	Validate::validate_and_update_metadata(&assets, &options);
	
	Ok(())
}

fn options() -> ValidateOptions {

	let args: Vec<String> = env::args().collect();

	fn has_arg(argument: &str, args: &Vec<String>) -> bool {
		args.iter().find(|a| a.as_str() == argument).is_some()
	}

	ValidateOptions {
		timestamps: has_arg("--timestamps", &args),
		contents: has_arg("--contents", &args),
		dry_run: has_arg("--dry-run", &args),
		log_level: if has_arg("--verbose", &args) {
			LogLevel::Verbose
		} else if has_arg("--minimal", &args) { 
			LogLevel::Minimal
		} else {
			LogLevel::Default
		},
	}
}