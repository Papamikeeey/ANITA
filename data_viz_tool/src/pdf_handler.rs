use pdf_extract::extract_text_from_mem;
use std::io::{self, Result};


pub fn parse_pdf_bytes(data: &[u8]) -> Result<String> {
	//Attempt to extract text from the byte slice using the pdf-extract library
	extract_text_from_mem(data)
		.map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
}

