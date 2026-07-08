mod asset;
mod crypto;
mod directory;
mod meta;
mod validate;

use validate::{LogLevel, Validate, ValidateOptions};

use clap::{Args, Parser, Subcommand};
use std::path::{PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	Validate(ValidateArgs),
	Update(UpdateArgs),
}

#[derive(Args, Debug)]
struct ValidateArgs {
	#[arg(long, default_value_t = false)]
	quick: bool,

	#[arg(long, default_value_t = false)]
	verbose: bool,

	#[arg(default_value = ".")]
	directory: PathBuf,
}

#[derive(Args, Debug)]
struct UpdateArgs {
	#[arg(long, default_value_t = false)]
	quick: bool,

	#[arg(long, default_value_t = false)]
	verbose: bool,

	#[arg(default_value = ".")]
	directory: PathBuf,
}

fn main() {
	let args = Cli::parse();

	let options = ValidateOptions {
		contents: match args.command {
			Commands::Validate(ref args) => !args.quick,
			Commands::Update(ref args) => !args.quick,
		},
		dry_run: match args.command {
			Commands::Validate(..) => true,
			Commands::Update(..) => false,
		},
		log_level: match args.command {
			Commands::Validate(ref args) => if args.verbose { LogLevel::Verbose } else { LogLevel::Default },
			Commands::Update(ref args) => if args.verbose { LogLevel::Verbose } else { LogLevel::Default },
		}
	};

	let directory = match args.command {
		Commands::Validate(ref args) => &args.directory,
		Commands::Update(ref args) => &args.directory,
	};

	println!("Options = {:?}", options);
	
	match args.command {
		Commands::Validate(..) => Validate::validate_assets(&directory, &options).unwrap(),
		Commands::Update(..) => Validate::update_assets(&directory, &options).unwrap(),
	};
	
}