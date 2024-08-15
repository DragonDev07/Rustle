use log::info;
use rayon::prelude::*;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::fs;
use std::io::Read;
use url::Url;
extern crate pretty_env_logger;

/// Represents a web crawler with a specified origin URL and recursion depth.
///
/// ## Fields
/// - `origin_url`: The starting URL for the crawler.
/// - `recursion_depth`: The maximum depth to which the crawler will run.
pub struct Crawler {
    origin_url: String,
    recursion_depth: u64,
}

impl Crawler {
    /// Creates a new instance of the `Crawler` struct.
    ///
    /// ## Arguments
    /// * `origin_url` - A `String` representing the starting URL of the crawler.
    /// * `recursion_depth` - A `u64` representing the maximum depth to which the crawler will run.
    /// ## Returns
    ///
    /// A new instance of the `Crawler` struct.
    pub fn new(origin_url: String, recursion_depth: u64) -> Self {
        Crawler {
            origin_url,
            recursion_depth,
        }
    }

    /// Starts the crawling process from the origin URL.
    ///
    /// This function initializes a reqwest blocking client, fetches the HTMl content of the origin
    /// URl, extracts all links from it, and iterates over these links to discover new links.
    pub fn crawl(&self) {
        // Declare reqwest blocking client
        let reqwest_client = reqwest::blocking::Client::new();

        // Get HTML of origin url
        let html = Self::get_html(&reqwest_client, &self.origin_url);
        Self::write_html("", &html);

        // Get all links from the origin url
        let urls = Self::get_links(&self, &html);

        // Iterate over all links until none are left
        Self::iterate_links(&self, &urls, &reqwest_client, 0);
    }

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
    fn get_links(&self, html: &str) -> HashSet<String> {
        return Document::from(html)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .filter_map(|url| self.normalize_url(url))
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
    fn normalize_url(&self, url: &str) -> Option<String> {
        let parsed_url = Url::parse(url);
        match parsed_url {
            Ok(parsed_url) => {
                if parsed_url.has_host() && parsed_url.host_str().unwrap() == self.origin_url {
                    return Some(url.to_string());
                } else {
                    return None;
                }
            }
            Err(_e) => {
                if url.starts_with("//") {
                    return Some(format!("https:{}", url));
                } else if url.starts_with('/') {
                    return Some(format!("{}{}", self.origin_url, url));
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
        &self,
        url: &String,
        reqwest_client: &reqwest::blocking::Client,
    ) -> HashSet<String> {
        let html = Self::get_html(&reqwest_client, url);
        let links = Self::get_links(&self, &html);

        let parsed_url = Url::parse(url).unwrap();
        let path = parsed_url.path();
        Self::write_html(path, &html);

        info!("Scraped {} - {} Links", url, links.len());

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
    fn iterate_links(
        &self,
        origin_links: &HashSet<String>,
        reqwest_client: &reqwest::blocking::Client,
        mut depth: u64,
    ) {
        let mut visited_urls = HashSet::new();
        visited_urls.insert(self.origin_url.to_string());

        let mut new_urls = origin_links
            .difference(&visited_urls)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        while !(depth >= self.recursion_depth) && !new_urls.is_empty() {
            let (next_visited_urls, next_new_urls): (HashSet<String>, HashSet<String>) = new_urls
                .par_iter()
                .map(|url| {
                    let links = Self::fetch_and_process_links(&self, &url, &reqwest_client);

                    return (url.clone(), links);
                })
                .fold(
                    || (HashSet::new(), HashSet::new()),
                    |(mut visited, mut new), (url, links)| {
                        visited.insert(url);
                        new.extend(links.difference(&visited).cloned());

                        return (visited, new);
                    },
                )
                .reduce(
                    || (HashSet::new(), HashSet::new()),
                    |(mut visited1, mut new1), (visited2, new2)| {
                        visited1.extend(visited2);
                        new1.extend(new2);
                        return (visited1, new1);
                    },
                );

            visited_urls.extend(next_visited_urls);
            new_urls = next_new_urls;
            depth += 1;
            info!("------ DEPTH: {} ------", depth);
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
}
