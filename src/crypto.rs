use std::fs::{File};
use std::io::{self, Write};
use std::error::Error;
use std::path::{PathBuf};
// use yaml_rust2::{YamlLoader, YamlEmitter};
use sha2::{Sha256, Digest};
use std::io::{BufRead, BufReader};

pub fn sha256_file(path: PathBuf) -> Result<String, Box<dyn Error>> {
	stream_sha256(path)
}

// TODO: Does not match linux sha256sum...try reading file as binary in chunks
fn stream_sha256(path: PathBuf) -> Result<String, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut hasher = Sha256::new();

	for line_result in reader.lines() {
		let l = line_result?;
		println!("{}", &l);
		hasher.update(&l);
	}

	let hash = hasher.finalize();
	let base64 = format!("{:X}", hash);

	Ok(base64)
}


fn write_file() -> io::Result<()> {
	let path = "output.txt";
	let mut file = File::create(path)?;

	// Write a byte slice directly
	file.write_all(b"Some data!\n")?;

	// Use the write! macro for formatted strings
	let name = "Rust";
	let version = "1.x";
	writeln!(file, "Hello, {} version {}!", name, version)?;

	Ok(())
}