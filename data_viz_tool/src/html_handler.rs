use select::document::Document;
use select::predicate::Name;

#[allow(dead_code)]
fn parse_html(html_content: &str) -> String {
	let document = Document::from(html_content);
	let body = document.find(Name("body")).next().unwrap().text();


	body
}