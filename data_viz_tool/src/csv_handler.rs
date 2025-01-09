use csv::ReaderBuilder;
use std::error::Error;
use std::io::Cursor;

// Parses CSV data from a byte slice (for in-memory CSV files).

pub fn parse_csv_bytes(data: &[u8]) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
	// Use a cursor to read the byte slice as an in-memory file.
	let cursor = Cursor::new(data);

	// Build a CSV reader.
	let mut reader = ReaderBuilder::new()
		.has_headers(true) // Set to 'false' if the file doesn't have headers.
		.from_reader(cursor);

	let mut rows = Vec::new();

	// Read records from the CSV reader.
	for result in reader.records() {
		let record = result?;
		rows.push(record.iter().map(String::from).collect());
	}

	Ok(rows)
}

// Parses a CSV file from a file path.
#[allow(dead_code)]
pub fn parse_csv_file(file_path: &str) -> Result <Vec<Vec<String>>, Box<dyn Error>> {
	// Open the file at the path specified.
	let file = std::fs::File::open(file_path)?;

	// Build a CSV Reader to take charge...
	let mut reader = ReaderBuilder::new()
		.has_headers(true) // Again, set it to 'false' if the file does not contain any headers...
		.from_reader(file);

	let mut rows = Vec::new();

	// Read the records from the CSV reader.
	for result in reader.records() {
		let record = result?;
		rows.push(record.iter().map(String::from).collect());
	}

	Ok(rows)
}