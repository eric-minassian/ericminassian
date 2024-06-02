use std::env;

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    /// Create a new Config.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the PORT environment variable is not a number.
    pub fn new() -> Self {
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").map_or(3000, |f| f.parse().expect("PORT must be a number"));

        Self { host, port }
    }
}
