use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

/// `JupiterClient` is a client wrapper to interact with the Jupiter Aggregator APIs.
/// It is your gateway to interact with the Jupiter exchange API
#[derive(Debug)]
pub struct JupiterClient {
    pub client: Client,
    pub base_url: String,
}

impl JupiterClient {
    /// Creates a new instance of `JupiterClient`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL for the Jupiter API, typically `https://lite-api.jup.ag`. for pro api use https://api.jup.ag
    ///
    /// # Example
    ///
    /// ```
    /// let api = JupiterClient::new("https://lite-api.jup.ag");
    /// ```
    pub fn new(base_url: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build client with API key");

        // let client = Client::new();
        JupiterClient {
            client,
            base_url: base_url.to_string(),
        }
    }

    /// Returns a new JupiterClient with the API key set in headers.
    ///
    /// # Arguments
    ///
    /// * `api_key` - your api key, you can get one from here `https://portal.jup.ag/onboard`.
    ///
    /// # Example
    ///
    /// ```
    /// let api = JupiterClient::new("https://api.jup.ag").with_api_key('your-api-key');
    /// ```
    pub fn with_api_key(self, api_key: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", HeaderValue::from_str(api_key).unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        headers.insert("Content-Type", "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build client with API key");

        JupiterClient {
            client,
            base_url: self.base_url,
        }
    }
}

// Include all the API method implementations
mod recurring_api;
mod swap_api;
mod token_api;
mod trigger_api;
mod ultra_api;
