mod json_handler;

use std::env;
use serde:: Deserialize;
use std::error::Error;
use std::fs::File;
use std::process;
use csv;


#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Record {
    year: u32,
    budget: String,
    percentage_of_projects_completed: String,
}



fn main() {

    println!("Current directory: {:?}", env::current_dir().unwrap());


    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }
    let file_path = &args[1];
    let extension = file_path.split('.').last().unwrap_or("");

    match extension {
        "csv" => {
            if let Err(err) = read_and_print_csv(file_path) {
                eprintln!("Error generatig example asked: {err}, ");
                process::exit(1);
            }
        },

        "json" => {
            if let Err(err) = json_handler::process_json(file_path) {
                println!("Error generating example asked for: {err}, ");
                process::exit(1);
            }
        },
        _ => {
            eprintln!("Unsupported file format");
            process::exit(1);
        }
    }

}



fn read_and_print_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);


    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())


}

#[allow(dead_code)]
fn parse_percentage(s: &str) -> Result<f32, Box<dyn Error>> {
    let trimmed = s.trim_end_matches('%');
    trimmed.parse::<f32>().map_err(|e| e.into())
}