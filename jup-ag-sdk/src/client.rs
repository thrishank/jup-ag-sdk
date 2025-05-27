use reqwest::Client;

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
    /// * `base_url` - Base URL for the Jupiter API, typically `https://lite-api.jup.ag/swap/v1`.
    ///
    /// # Example
    ///
    /// ```
    /// let api = JupiterClient::new("https://lite-api.jup.ag");
    /// ```
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        JupiterClient {
            client,
            base_url: base_url.to_string(),
            // TODO: Add Api key here
            // make the base_url default
        }
    }
}

// Include all the API method implementations
mod swap_api;
mod token_api;
mod trigger_api;
mod ultra_api;
