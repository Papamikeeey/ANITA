use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;


#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
	year: u32,
	budget: String,
	percentage_of_projects_completed: String,
}

pub fn process_json(file_path: &str) -> std::result::Result<(), Box<dyn Error>> {
	let data = fs::read_to_string(file_path)?;
	let records: Vec<Record> = serde_json::from_str(&data)?;
	for record in records {
		println!("{:?}", record);
	}
	Ok(())
}
