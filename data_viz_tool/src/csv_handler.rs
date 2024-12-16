use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;


pub fn parse_csv(file_path: &str) ->Result<(), Box<dyn Error>> {
	// Open the CSV file
	let file = File::open(file_path)?;

	// Create a CSV reader with flexible settings
	let mut rdr = ReaderBuilder::new()
		.flexible(true) // Allow rows with varying lengths
		.trim(csv::Trim::All) // Remove surrounding whitespace
		.comment(Some(b'#')) // Skip lines starting with a '#'
		.has_headers(false) // Do not assume headers
		.from_reader(file);


	println!("Parsing CSV file...");

	let mut is_first_row = true; // Initialize first row detection

	for result in rdr.records() {
		match result {
			Ok(record) => {
				if is_first_row {
					println!("Detected Headers (or first row): {:?}", record);
					is_first_row = false;
				} else {
					println!("Row: {:?}", record);
				}
			}
			Err(e) => {
				eprintln!("Error reading row: {}", e);
			}
		}
	}

	println!("Finished parsing CSV file.");
	Ok(())
}