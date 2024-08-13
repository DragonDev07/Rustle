struct Crawler {
    origin_url: String,
    recursion_depth: u64,
}

impl Crawler {
    fn new(origin_url: String, recursion_depth: u64) -> Self {
        Crawler {
            origin_url,
            recursion_depth,
        }
    }
}
