use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::time::Instant;
use url::Url;

/// The base URL to start web crawling from.
///
/// This constant defines the origin URL where the web crawler begins its operation.
/// All links extracted and processed will be relative to this origin URL.
///
/// ## Example
///
/// ```
/// pub const ORIGIN_URL: &str = "https://wikipedia.org";
/// ```
pub const ORIGIN_URL: &str = "https://wikipedia.org";

/// Fetches the HTML content of the given URL using the provided reqwest blocking client.
///
/// This function sends a GET request to the specified URL and reads the response body into a string.
///
/// ## Arguments
///
/// * `reqwest_client` - A reference to the reqwest blocking client used to make the HTTP request.
/// * `url` - A string slice that holds the URL to be fetched.
///
/// ## Returns
///
/// A `String` containing the HTML content of the given URL.
fn get_html(reqwest_client: &reqwest::blocking::Client, url: &str) -> String {
    let mut site = reqwest_client.get(url).send().unwrap();
    let mut html = String::new();
    site.read_to_string(&mut html).unwrap();

    return html;
}

/// Extracts and normalizes all the links from the given HTML content.
///
/// This function parses the HTML content, finds all anchor (`<a>`) tags, and extracts their `href` attributes.
/// It then normalizes these URLs using the `normalize_url` function and collects them into a `HashSet`.
///
/// ## Arguments
///
/// * `html` - A string slice that holds the HTML content to be processed.
///
/// ## Returns
///
/// A `HashSet<String>` containing all the normalized links found in the HTML content.
fn get_links(html: &str) -> HashSet<String> {
    return Document::from(html)
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(normalize_url)
        .collect::<HashSet<String>>();
}

/// Normalizes a given URL to ensure it is a valid and complete URL.
///
/// This function attempts to parse the given URL and checks if it has a host that matches the `ORIGIN_URL`.
/// If the URL is relative (starts with `//` or `/`), it will be converted to an absolute URL using `ORIGIN_URL`.
///
/// ## Arguments
///
/// * `url` - A string slice that holds the URL to be normalized.
///
/// ## Returns
///
/// An `Option<String>` containing the normalized URL if it is valid and belongs to the same host as `ORIGIN_URL`,
/// otherwise `None`.
fn normalize_url(url: &str) -> Option<String> {
    let parsed_url = Url::parse(url);
    match parsed_url {
        Ok(parsed_url) => {
            if parsed_url.has_host() && parsed_url.host_str().unwrap() == ORIGIN_URL {
                return Some(url.to_string());
            } else {
                return None;
            }
        }
        Err(_e) => {
            if url.starts_with("//") {
                return Some(format!("https:{}", url));
            } else if url.starts_with('/') {
                return Some(format!("{}{}", ORIGIN_URL, url));
            } else {
                return None;
            }
        }
    }
}

/// Fetches the HTML content of the given URL and extracts all the links from it.
///
/// ## Arguments
///
/// * `url` - A string slice that holds the URL to be fetched.
/// * `reqwest_client` - A reference to the reqwest blocking client used to make the HTTP request.
///
/// ## Returns
///
/// A `HashSet<String>` containing all the links extracted from the HTML content of the given URL.
fn fetch_and_process_links(
    url: &String,
    reqwest_client: &reqwest::blocking::Client,
) -> HashSet<String> {
    let html = get_html(&reqwest_client, url);
    let links = get_links(&html);

    let parsed_url = Url::parse(url).unwrap();
    let path = parsed_url.path();
    write_html(path, &html);

    println!("Scraped {} - {} Links", url, links.len());

    return links;
}

/// Iterates through the given set of origin links, fetching and processing each link to discover new links.
///
/// This function maintains a set of visited URLs to avoid processing the same URL multiple times.
/// It uses a reqwest blocking client to fetch the HTML content of each URL and extracts links from it.
/// The process continues until there are no new URLs to visit.
///
/// ## Arguments
///
/// * `origin_links` - A reference to a `HashSet<String>` containing the initial set of URLs to start the iteration.
/// * `reqwest_client` - A reference to the reqwest blocking client used to make the HTTP requests.
fn iterate_links(origin_links: &HashSet<String>, reqwest_client: &reqwest::blocking::Client) {
    let mut visited_urls = HashSet::new();
    visited_urls.insert(ORIGIN_URL.to_string());

    let mut new_urls = origin_links
        .difference(&visited_urls)
        .map(|x| x.to_string())
        .collect::<HashSet<String>>();

    while !new_urls.is_empty() {
        let (next_visited_urls, next_new_urls) = new_urls.iter().fold(
            (visited_urls.clone(), HashSet::new()),
            |(mut visited, mut new), url| {
                let links = fetch_and_process_links(&url, &reqwest_client);
                visited.insert(url.clone());
                new.extend(links.difference(&visited).cloned());
                (visited, new)
            },
        );

        visited_urls = next_visited_urls;
        new_urls = next_new_urls;
    }
}

/// Writes the HTML content to a file organized by the given path.
///
/// This function creates directories based on the provided path and writes the HTML content
/// to a file named `index.html` within those directories. The base directory is `static`.
///
/// ## Arguments
///
/// * `path` - A string slice that holds the path where the HTML content will be saved.
/// * `html_content` - A string slice that holds the HTML content to be written to the file.
fn write_html(path: &str, html_content: &str) {
    fs::create_dir_all(format!("static{}", path)).unwrap();
    let _ = fs::write(format!("static{}/index.html", path), html_content);
}

/// The main function that initializes the web crawler.
///
/// This function performs the following steps:
/// 1. Declares a reqwest blocking client.
/// 2. Fetches the HTML content of the origin URL.
/// 3. Writes the HTML content of the origin URL to a file.
/// 4. Extracts all links from the origin URL's HTML content.
/// 5. Iterates over all extracted links, fetching and processing each link to discover new links.
///
/// The HTML content of each URL is written to a file organized by the URL's path.

fn main() {
    // Start Runtime
    let time = Instant::now();

    // Declare reqwest blocking client
    let reqwest_client = reqwest::blocking::Client::new();

    // Get HTML of origin url
    let html = get_html(&reqwest_client, ORIGIN_URL);
    write_html("", &html);

    // Get all links from the origin url
    let urls = get_links(&html);

    // Iterate over all links until none are left
    iterate_links(&urls, &reqwest_client);

    println!("Runtime: {}", time.elapsed().as_secs());
}
