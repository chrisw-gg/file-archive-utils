use std::error::{Error};
use std::fs;
use std::path::{PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MetaData {
	pub id: String,
	pub name: String,
	pub sha256: String,
}

pub fn read(path: &PathBuf) -> Result<MetaData, Box<dyn Error>> {
	let contents = fs::read_to_string(path)?;
	let meta_data = serde_yaml_ng::from_str(&contents)?;

	Ok(meta_data)
}

pub fn write(meta_data: &MetaData, path: &PathBuf) -> Result<(), Box<dyn Error>> {
	let yaml = serde_yaml_ng::to_string(&meta_data)?;
	fs::write(path, yaml)?;

	Ok(())
}
