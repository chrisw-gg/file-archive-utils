mod asset;
mod crypto;
mod directory;
mod meta;
mod validate;

use clap::Parser;
use std::env;
use std::error::{Error};
use std::path::{PathBuf};

use asset::{Assets};
use validate::{LogLevel, Validate, ValidateOptions};

use crate::validate::LogLevel::Default;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
	#[arg(long, default_value_t = false)]
	contents: bool,

	#[arg(long, default_value_t = true)]
	dry_run: bool,

	#[arg(long, default_value_t = false)]
	verbose: bool,

	#[arg(default_value = ".")]
	directory: PathBuf,
}

fn main() {
	let args = Args::parse();

	let options = ValidateOptions {
		contents: args.contents,
		dry_run: args.dry_run,
		log_level: if args.verbose { LogLevel::Verbose } else { LogLevel::Default }
	};
	
	let assets = Assets::new(&args.directory).unwrap();

	Validate::validate_and_update_metadata(&assets, &options);
	
}