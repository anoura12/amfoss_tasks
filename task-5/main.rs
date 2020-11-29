use error_chain::error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.worldometers.info/coronavirus").await?.text().await?;


    let document = Document::from_read(res.as_bytes()).unwrap();

    for node in document
        .find(Class("total_row_world"))
        .take(10)
{
    let country = node.find(Class("mt_a"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();

    println!("{}", country.join(","));
