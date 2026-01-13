mod asset;
mod update;
mod validate;

use std::env;
use std::error::{Error};

use asset::{Assets};
use validate::{Validate, ValidateOptions};

fn main() {
	_ = run();
}

fn run() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {

		let command = &args[1];

		let options = ValidateOptions {
			timestamps: args.iter().find(|a| a.as_str() == "--timestamps").is_some(),
			contents: args.iter().find(|a| a.as_str() == "--contents").is_some(),
		};

		let assets = Assets::new().unwrap();
		
		// readonly
		if command == "report" {
			report()?; // TODO: Even need this???
		} else {

			let results = Validate::validate(&assets, &options);
			Validate::print_results(results);

			
		}
	}
	
	Ok(())
}

fn report() -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	assets.print_report();
	Ok(())
}

fn update(quick: bool) -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	assets.print_report();
	Ok(())
}
