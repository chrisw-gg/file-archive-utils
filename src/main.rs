mod asset;
mod crypto;
mod directory;
mod meta;
mod update;
mod validate;

use std::env;
use std::error::{Error};

use asset::{Assets};
use update::{Update};
use validate::{Validate, ValidateOptions};

fn main() {
	_ = run();
}

fn run() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	//if args.len() < 2 {
	//	panic!("No command specified");
	//}

	//let command = &args[1];

	let options = ValidateOptions {
		timestamps: true, // always compare timestamps (contents will just potentially override this)
		contents: args.iter().find(|a| a.as_str() == "--contents").is_some(),
		dry_run: args.iter().find(|a| a.as_str() == "--dry-run").is_some(),
		detailed: args.iter().find(|a| a.as_str() == "--detailed").is_some(),
	};

	let assets = Assets::new().unwrap();
		
	let validation_results = Validate::validate(&assets, &options);
	
	if options.detailed {
		Validate::print_results(&validation_results);
	}

	Update::update_meta_files(&validation_results, &options);
	
	Ok(())

}