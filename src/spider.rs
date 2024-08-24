use crate::database::Database;
use crate::domain::Domain;
use crate::site::Site;
use anyhow::Result;
use chrono::Utc;
use log::{info, trace, warn};
use rayon::prelude::*;
use robots_txt::matcher::SimpleMatcher;
use robots_txt::Robots;
use select::document::Document;
use select::predicate::Name;
use std::collections::HashSet;
use std::io::Read;
use url::Url;
extern crate pretty_env_logger;

/// Represents a web crawler with a specified origin URL and recursion depth.
pub struct Crawler {
    /// The starting URL for the crawler.
    origin_url: String,
    /// The maximum depth to which the crawler will run.
    recursion_depth: u64,
    /// The database that the crawler will store sites in.
    database: Database,
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
    pub fn new(origin_url: String, recursion_depth: u64, database_name: &str) -> Result<Self> {
        Ok(Crawler {
            origin_url,
            recursion_depth,
            database: Database::new(database_name)?,
        })
    }

    /// Starts the crawling process from the origin URL.
    ///
    /// This function initializes a reqwest blocking client, fetches the HTMl content of the origin
    /// URl, extracts all links from it, and iterates over these links to discover new links.
    pub fn crawl(&self) {
        info!(
            "Starting crawl process from origin URL: {}",
            self.origin_url
        );

        // Declare reqwest blocking client
        let reqwest_client = reqwest::blocking::Client::new();

        // Setup Database
        let _ = self.database.setup();

        // Get HTML of origin url
        let html = match Self::get_html(&reqwest_client, &self.origin_url) {
            Some(content) => content,
            None => {
                warn!("Skipping URL with unsupported scheme: {}", self.origin_url);
                return;
            }
        };

        // Get all links from the origin url
        let urls = Self::get_links(&self, &html);

        // Save origin URL to database
        Self::write_site(&self, &self.origin_url, &urls);

        // Fetch and store robots.txt
        let domain = Url::parse(&self.origin_url)
            .unwrap()
            .host_str()
            .unwrap()
            .to_string();
        if let Some(robots) = self.get_robots(&domain, &reqwest_client) {
            Self::write_domain(&self, &domain, &robots);
        }

        // Iterate over all links until none are left
        Self::iterate_links(&self, &urls, &reqwest_client, 0);

        // Print Database Summary
        let _ = Site::summarize_site_table(&self.database);
        let _ = Domain::summarize_domain_table(&self.database);
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
    fn get_html(reqwest_client: &reqwest::blocking::Client, url: &str) -> Option<String> {
        trace!("Fetching HTML content for URL: {}", url);

        // Parse the URL to check its scheme
        let parsed_url = Url::parse(url).unwrap();
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            warn!("Unsupported URL scheme {}", parsed_url.scheme());
            return None;
        }

        // Fetch the site and make sure it accepts connection
        let response = reqwest_client.get(url).send();
        let mut site = match response {
            Ok(resp) => resp,
            Err(e) => {
                warn!("Failed to fetch URL: {}: {}", url, e);
                return None;
            }
        };

        // Fetch HTML content
        let mut html = String::new();
        if let Err(e) = site.read_to_string(&mut html) {
            warn!(
                "Failed to read response as valid UTF-8 for URL: {}: {}",
                url, e
            );
            return None;
        }

        return Some(html);
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
        trace!("Extracting links from HTML content");
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
        trace!("Normalizing URL: {}", url);

        // Parse the Url with the `Url` crate
        let parsed_url = Url::parse(url);
        match parsed_url {
            // If the parsed Url is a valid Url
            Ok(parsed_url) => {
                // If its host matched the origin url, return it, else, skip it
                if parsed_url.has_host() {
                    return Some(url.to_string());
                } else {
                    return None;
                }
            }
            // If the parsed Url is not a valid Url
            Err(_e) => {
                // If the Url starts with "//" (relative top level Url), normalize it with https
                // If the Url starts with "/" (relative path Url), normalize it with the origin url
                // Else, skip the Url
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
        trace!("Fetching and processing links for URL: {}", url);

        // Get HTML from given URL
        let html = match Self::get_html(&reqwest_client, url) {
            Some(content) => content,
            None => {
                warn!("Skipping URL with unsupported scheme: {}", url);
                return HashSet::new();
            }
        };

        // Extract links from the HTML
        let links = Self::get_links(&self, &html);

        // Write Url to Database
        Self::write_site(&self, url, &links);

        trace!("Scraped {} - {} Links", url, links.len());

        return links;
    }

    /// Checks if a URL exists in the database and if its crawl_time is less than a day old, skips
    /// it.
    ///
    /// ## Arguments
    ///
    /// * `url` - A string slice that holds the URL of the site.
    ///
    /// ## Returns
    ///
    /// A boolean indicating whether the URL should be skipped.
    pub fn should_skip_cached_url(&self, url: &str) -> Result<bool> {
        if let Some(site) = Site::read_into(url, &self.database)? {
            let one_day_ago = Utc::now() - chrono::Duration::days(1);
            if site.crawl_time > one_day_ago {
                trace!("Skipping cached URL: {}", url);
                return Ok(true);
            }
        }

        return Ok(false);
    }

    /// Checks if a URL is allowed to be scraped based on the robots.txt rules.
    ///
    /// ## Arguments
    ///
    /// * `url` - A string slice that holds the URL to be checked.
    ///
    /// ## Returns
    ///
    /// A boolean indicating whether the URL is allowed to be scraped.    
    fn is_allowed_to_scrape(&self, url: &str) -> Result<bool> {
        let parsed_url = Url::parse(url).unwrap();
        let path = parsed_url.path().to_string();
        let domain = parsed_url.host_str().unwrap().to_string();

        // Check if robots.txt is already in the database
        let robots_txt = if let Some(domain_data) = Domain::read_into(&domain, &self.database)? {
            domain_data.robots
        } else {
            // Fetch robots.txt from the domain
            let robots = self.get_robots(&domain, &reqwest::blocking::Client::new());
            if let Some(robots_content) = robots {
                // Save robots.txt to the database
                self.write_domain(&domain, &robots_content);
                robots_content
            } else {
                String::new()
            }
        };

        // Parse robots.txt and check if the URL is allowed
        let robots = Robots::from_str_lossy(&robots_txt);
        let matcher = SimpleMatcher::new(&robots.choose_section("Rustle").rules);
        let allowed = matcher.check_path(&path);

        trace!("URL: {} - Allowed? {}", url, allowed);

        return Ok(allowed);
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
        info!(
            "Starting link iteration with target depth: {}",
            self.recursion_depth
        );

        // Initialize a set to keep track of visited URLs
        let mut visited_urls = HashSet::new();
        visited_urls.insert(self.origin_url.to_string());

        // Fetch new set of URLs to visit, exlcuding visited URLs
        let mut new_urls = origin_links
            .difference(&visited_urls)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();

        // Loop until the maximum recursion depth is reached, or there are no new URLs to visit
        while !(depth >= self.recursion_depth) && !new_urls.is_empty() {
            // Use parallel iteration w/ `rayon` crate to process URLs
            let (next_visited_urls, next_new_urls): (HashSet<String>, HashSet<String>) = new_urls
                .par_iter()
                .map(|url| {
                    // Check if site is cached and can be skipped
                    if self.should_skip_cached_url(url).unwrap()
                        && !self.is_allowed_to_scrape(url).unwrap()
                    {
                        return None;
                    }

                    // Fetch all links from the current URL
                    let links = Self::fetch_and_process_links(&self, &url, &reqwest_client);

                    return Some((url.clone(), links));
                })
                .fold(
                    // Inititalize empty sets for visited and new URLs
                    || (HashSet::new(), HashSet::new()),
                    |(mut visited, mut new), opt| {
                        if let Some((url, links)) = opt {
                            // Add the current URl to the visited set
                            visited.insert(url);

                            // Add all newly found links to the new set, exclduing already visited URLs
                            new.extend(links.difference(&visited).cloned());
                        }
                        return (visited, new);
                    },
                )
                .reduce(
                    // Combine results from different threads
                    || (HashSet::new(), HashSet::new()),
                    |(mut visited1, mut new1), (visited2, new2)| {
                        visited1.extend(visited2);
                        new1.extend(new2);
                        return (visited1, new1);
                    },
                );

            // Update loop variables
            visited_urls.extend(next_visited_urls);
            new_urls = next_new_urls;
            depth += 1;
            trace!("------ DEPTH: {} ------", depth);
        }
    }

    /// Fetches the `robots.txt` file for a given domain.
    ///
    /// ## Arguments
    ///
    /// * `domain` - A string slice that holds the domain name.
    /// * `reqwest_client` - A reference to a `reqwest::blocking::Client` used to make the HTTP request.
    ///
    /// ## Returns
    ///
    /// An `Option<String>` which contains the content of the `robots.txt` file if the request is successful,
    /// or `None` if the request fails or the file does not exist.
    pub fn get_robots(
        &self,
        domain: &str,
        reqwest_client: &reqwest::blocking::Client,
    ) -> Option<String> {
        let robots_url = format!("https://{}/robots.txt", domain);
        match reqwest_client.get(&robots_url).send() {
            Ok(response) => {
                if response.status().is_success() {
                    return response.text().ok();
                }
            }
            Err(e) => {
                trace!("Failed to fetch robots.txt for {}: {}", domain, e);
            }
        }
        return None;
    }

    /// Writes a `Site` to the database.
    ///
    /// This function creates a `Site` instance with the given URL and links,
    /// sets the current time as the crawl time, and writes the `Site` to the database.
    ///
    /// ## Arguments
    ///
    /// * `url` - A string slice that holds the URL of the site.
    /// * `links_to` - A reference to a `HashSet` containing the URLs that the site links to.
    fn write_site(&self, url: &str, links_to: &HashSet<String>) {
        trace!("Writing site to database for URL: {}", url);

        // Declare a `Site` struct to hold information
        let site = Site {
            url: url.to_string(),
            crawl_time: Utc::now(),
            links_to: links_to.clone(),
        };

        // Call method to write Site struct to database
        site.write_into(&self.database);
    }

    /// Writes a `Domain` to the database.
    ///
    /// This function creates a `Domain` instance with the given domain and robots.txt string,
    /// sets the current time as the crawl time, and writes the `Domain` to the database.
    ///
    /// ## Arguments
    ///
    /// * `domain` - A string slice that holds the domain of the site. (Fomratted as "example.com")
    /// * `robots` - A string slice that holds the contents of the domain's robots.txt
    fn write_domain(&self, domain: &str, robots: &str) {
        trace!("Writing domain to database for domain: {}", domain);

        let domain = Domain {
            domain: domain.to_string(),
            crawl_time: Utc::now(),
            robots: robots.to_string(),
        };

        domain.write_into(&self.database);
    }
}
