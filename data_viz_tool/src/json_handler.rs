use serde_json::Value;
use std::error::Error;


// Function to proces and display JSON data
pub fn process_json_bytes(data: &[u8]) -> Result<(), Box<dyn Error>> {
	// Read the JSON bytes into a serde_json::Value
	let json: Value = serde_json::from_slice(data)?;


	// Pretty -print the JSON structure
	println!("Parsed JSON data:");
	pretty_print_json(&json, 0);

	Ok(())
}


// Helper function to recursively pretty-print JSON data
fn pretty_print_json(value: &Value, indent_level: usize) {
	let indent = " ".repeat(indent_level); // Indentation for readability


	match value {
		Value::Object(map) => {
			println!("{}{{", indent);
			let mut entries = map.iter().peekable();
			while let Some((key, val)) = entries.next() {
				println!("\n{}  \"{}\": ", indent, key);
				pretty_print_json(val, indent_level + 2);
				if entries.peek().is_some() {
					println!(",");
				}
			}
			println!();
			println!("{}}}", indent);
		}
		Value::Array(arr) => {
			println!("{}[", indent);
			let mut items = arr.iter().peekable();
			while let Some(val) = items.next() {
				pretty_print_json(val, indent_level + 2);
				if items.peek().is_some() {
					println!(", ");
				}
			}
			println!("]");
		}
		Value::String(s) => println!("\"{}\"", s),
		Value::Number(n) => println!("{}", n),
		Value::Bool(b) => println!("{}", b),
		Value::Null => println!("null"),
	}

}



