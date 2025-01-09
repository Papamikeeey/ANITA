use reqwest;
use std::error::Error;

pub async fn fetch_and_parse_url(url: &str) -> Result<String, Box<dyn Error>> {
    // Make an HTTP GET request to fetch the URL's content
    let response = reqwest::get(url).await?.text().await?;

    // Call the HTML handler to parse the HTML content
    let parsed_text = crate::html_handler::parse_html_bytes(&response.as_bytes())?;


    // Return the extracted text
    Ok(parsed_text)
}