use select::document::Document;
use select::predicate::Name;
use std::io::{self, Result};

pub fn parse_html_bytes(html_content: &[u8]) -> Result<String> {

	// Convert the byte slice to a string
	let content = String::from_utf8(html_content.to_vec())
		.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

	// Parse the HTML content using the 'select' library
	let document = Document::from(content.as_str());

	// Extract the <body> text if it exists
	if let Some(body) = document.find(Name("body")).next() {
		Ok(body.text())
	} else {
		Err(io::Error::new(io::ErrorKind::InvalidData, "No <body> tage found in HTML"))
	}

}