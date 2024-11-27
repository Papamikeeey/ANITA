use pdf_extract::extract_text;
use std::path::Path;
// use std::error::Error;
use std::io::{self, Result};


pub fn extract_text_from_pdf(file_path: &Path) -> Result<String> {
	//Attempt to extract text using the pdf-extract library
	extract_text(file_path)
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

