use docx_rs::Docx;
//use std::error::Error;
use std::fs;
use std::io::Read;


pub fn parse_docx(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
	
	let mut buf = Vec::new();
	let mut file = fs::File::open(file_path)?;
	file.read_to_end(&mut buf)?;


	let _docx = Docx::new();


	let text = "";



	Ok (text.to_string())
}