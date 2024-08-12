use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::io::Read;
use url::Url;

pub const ORIGIN_URL: &str = "https://wikipedia.org";

fn links_from_html(html: &str) -> HashSet<String> {
    Document::from(html)
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(normalize_url)
        .collect::<HashSet<String>>()
}

fn normalize_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url);
    match parsed_url {
        Ok(parsed_url) => {
            if parsed_url.has_host() && parsed_url.host_str().unwrap() == ORIGIN_URL {
                Some(url.to_string())
            } else {
                None
            }
        }
        Err(_e) => {
            if url.starts_with("//") {
                Some(format!("https:{}", url))
            } else if url.starts_with('/') {
                Some(format!("{}{}", ORIGIN_URL, url))
            } else {
                None
            }
        }
    }
}

fn main() {
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(ORIGIN_URL).send().unwrap();
    println!("Status for {}: {}", ORIGIN_URL, res.status());

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let urls = links_from_html(&body);
    println!("URLs: {:#?}", urls);
}
