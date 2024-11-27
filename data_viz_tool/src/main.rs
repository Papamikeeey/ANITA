mod json_handler;
mod pdf_handler;
mod docx_handler;
mod html_handler;
mod url_handler;

use std::env;
use serde:: Deserialize;
use std::error::Error;
use std::process;
use std::path::Path;
use std::fs::File;




#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Record {
    year: u32,
    budget: String,
    percentage_of_projects_completed: String,
}



fn main() -> Result<(), Box<dyn Error>> {
    println!("Current directory: {:?}", env::current_dir()?);
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1); 
    }
    let file_path = Path::new(&args[1]);
    let extension = file_path.extension().and_then(std::ffi::OsStr::to_str).unwrap_or("");

    match extension {
        "csv" => {
            read_and_print_csv(file_path.to_str().unwrap())?;
        },
        "json" => {
            json_handler::process_json(file_path.to_str().unwrap())?
        },
        "pdf" => {
            let text = pdf_handler::extract_text_from_pdf(file_path)?;
            println!("Extracted text from PDF: {}", text);
        },
        "docx" => {
            let text = docx_handler::parse_docx(file_path.to_str().unwrap())?;
            println!("Extracted from DOCX: {}", text);
        },
        _ => {
            eprintln!("Unsupported file format");
            process::exit(1);
        }
    }


    Ok(())
}

fn read_and_print_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    use csv::Reader;
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }


    Ok(())
}
