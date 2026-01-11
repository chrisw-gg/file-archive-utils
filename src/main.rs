mod asset;
mod validate;

use std::env;
use std::error::{Error};

use asset::{Assets};
use validate::{Validate};

fn main() {
	_ = run();
}

fn run() -> Result<(), Box<dyn Error>> {

	let args: Vec<String> = env::args().collect();

	// The rest of the elements are the actual command line arguments/flags
	if args.len() > 1 {

		let command = &args[1];
		
		// readonly
		if command == "report" {
			report()?;
		}

		// readonly
		if command == "validate" {
			validate()?;
		}

		// !!!!
		if command == "update" {
			update()?;
		}
	}
	
	Ok(())
}

fn report() -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	assets.print_report();
	Ok(())
}

fn validate() -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	let results = Validate::validate(&assets);
	Validate::print_results(results);
	Ok(())
}

fn update() -> Result<(), Box<dyn Error>> {
	let assets = Assets::new().unwrap();
	assets.print_report();
	Ok(())
}

/*

	//let asset_folder = asset::gather(String::from("/workdir/foo"));

	//asset::Folder::walk_files(&asset_folder.unwrap().borrow(), |asset| {
	//	println!("{}", asset.id.display());
	//});


	while true {



		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");		

	}

// insert (only adds new .meta files)
fn insert(asset_folder: &asset::Folder) {

	asset_folder.print();

	println!("Continue? (yes)");

	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");

	if input == "yes" {

		

	}

}
*/
// Print (only print - no updates)

// Validate (runs sha256 on each file and compares to .meta file)