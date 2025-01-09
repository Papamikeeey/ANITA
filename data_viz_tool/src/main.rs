mod json_handler;
mod pdf_handler;
mod docx_handler;
mod html_handler;
mod url_handler;
mod csv_handler;
mod handle_file_upload;

use axum::{
    routing::{get, post},
    Router,
    extract::Json,
    response::Html,
    http::StatusCode,
};

use std::net::SocketAddr;
use serde::Deserialize;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define the routes for your Axum application
    let app = Router::new()
        .route("/", get(root)) // Home Route
        .route("/process-url", post(process_url)) // Route to handle URL processing
        .route("/upload-file", post(handle_file_upload::handle_file_upload)); // Route to handle file uploads

    // Define the address to run your server on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
        Ok(())
}


// Route handler for the home route
async fn root() -> Html<&'static str> {
    Html("<h1>Welcome to Amazing ANITA - The Data-to-Disney Animation Tool</h1>")
}


// Route handler for processing URLs
#[derive(Deserialize)]
struct UrlInput {
    url: String,
}

async fn process_url(Json(payload): Json<UrlInput>) -> Result<String, StatusCode> {
    let url = payload.url;

    if url.starts_with("http://") || url.starts_with("https://") {
        match url_handler::fetch_and_parse_url(&url).await {
            Ok(parsed_text) => Ok(format!("parsed text from URL: {}", parsed_text)),
            Err(_e) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
