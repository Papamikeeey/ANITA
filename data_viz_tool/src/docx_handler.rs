use std::io::{Cursor, Read};
use zip::ZipArchive;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::error::Error;


pub fn parse_docx_bytes(data: &[u8]) -> Result<String, Box<dyn Error>> {
	// Use a cursor to read the byte slice as a file
	let cursor = Cursor::new(data);
	let mut zip = ZipArchive::new(cursor)?;

	// Locate and read the 'word/document.xml' file
	let mut document_xml = zip.by_name("word/document.xml")?;
	let mut xml_content = String::new();
	document_xml.read_to_string(&mut xml_content)?;


	// Parse the XML to extract text
	let mut reader = Reader::from_str(&xml_content);
	reader.trim_text(true);
	let mut text = String::new();


	// Iterate through XML events and extract text
	loop {
		match reader.read_event() {
			Ok(Event::Text(e)) => {
				text.push_str(&e.unescape()?.to_string());
				text.push(' '); //Add a space in between the words
			}
			Ok(Event::Eof) => break, //End of File
			Err(e) => return Err(Box::new(e)),
			_ => {} //Ignore other events
		}
	}

	Ok(text)

}