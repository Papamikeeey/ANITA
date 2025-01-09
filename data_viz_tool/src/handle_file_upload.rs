use axum::{
    extract::Multipart,
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::{csv_handler, json_handler, pdf_handler, docx_handler, html_handler, url_handler};

pub async fn handle_file_upload(mut multipart: Multipart) -> Result<Json<serde_json::Value>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("unknown").to_string();
        let file_name = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field.content_type().unwrap_or("unknown").to_string();


        println!("Recieved file: {} (field name: {}. type: {})", file_name, name, content_type);

        // Read the file bytes
        let data = field.bytes().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Process the file based on its content type or extension
        if content_type.contains("csv") {
            println!("processing CSV files...");
            csv_handler::parse_csv_bytes(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if content_type.contains("json") {
            println!("Processing JSON files...");
            json_handler::process_json_bytes(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if content_type.contains("pdf") {
            println!("Processing PDF files...");
            pdf_handler::parse_pdf_bytes(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if content_type.contains("vnd.openxmlformats-officedocument.wordprocessingml.document") {
            println!("Processing DOCX files...");
            docx_handler::parse_docx_bytes(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if content_type.contains("html") {
            println!("Processing HTML files...");
            html_handler::parse_html_bytes(&data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else if content_type.contains("text/html") {
            println!("Processing URL or HTML files...");
            url_handler::fetch_and_parse_url("https://example.com")
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else {
            println!("Sorry, the file type is unsupported: {}", content_type);
            return Err(StatusCode::UNSUPPORTED_MEDIA_TYPE);
        }
    }

    Ok(Json(json!({ "status": "success", "message": "Files processed successfully! Yippeee :) "})))
}