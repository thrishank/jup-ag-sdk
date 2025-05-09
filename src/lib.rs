use reqwest::{Client, StatusCode};
use types::{QuoteRequest, QuoteResponse, SwapInstructions, SwapRequest, SwapResponse};
pub mod types;

/// `JupiterClient` is a client wrapper to interact with the Jupiter Aggregator APIs.
/// It is your gateway to interact with the Jupiter exchange API
pub struct JupiterClient {
    client: Client,
    base_url: String,
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
    /// let api = JupiterClient::new("https://lite-api.jup.ag/swap/v1");
    /// ```
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        JupiterClient {
            client,
            base_url: base_url.to_string(),
        }
    }

    /// Fetches a token swap quote from Jupiter based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - A [`QuoteRequest`] containing query parameters like mint addresses, amount, slippage, and more.
    ///
    /// # Returns
    ///
    /// * `Ok(QuoteResponse)` on success.
    /// * `Err` with error details if the request or deserialization fails.
    ///
    /// # Jupiter API Reference
    ///
    /// - [Quote Endpoint](https://dev.jup.ag/docs/api/swap-api/quote)
    ///
    /// # Example
    ///
    /// ```
    /// let inputMint = "So11111111111111111111111111111111111111112";
    /// let outputMint = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
    /// let amount = 1_000_000_000; // 1 SOL
    /// let req = QuoteRequest::new(inputMint, outputMint, amount);
    /// let quote = api.get_quote(req).await?;
    /// ```
    pub async fn get_quote(
        &self,
        params: QuoteRequest,
    ) -> Result<QuoteResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let url = self
            .client
            .get(format!("{}/quote", &self.base_url))
            .query(&params);

        println!("URL: {:?}", url);

        let response = match self
            .client
            .get(format!("{}/quote", &self.base_url))
            .headers(headers)
            .query(&params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(JupiterClientError::ApiError(error_text, status));
        }

        match response.json::<QuoteResponse>().await {
            Ok(quote_response) => Ok(quote_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Fetches a swap transaction from Jupiter's `/swap` endpoint.
    ///
    /// # Arguments
    /// * `data` - The [`SwapRequest`]payload.
    ///
    /// # Returns
    /// A `Result` containing the `SwapResponse` with the  base64-encoded unsigned transaction or an error.
    ///
    /// # Example
    /// ```
    /// let payload = SwapRequest::new("YourPubKey...", quote);
    /// let swap_transaction = api.get_swap_transaction(payload).await?;
    /// ```
    pub async fn get_swap_transaction(
        &self,
        data: SwapRequest,
    ) -> Result<SwapResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/swap", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(JupiterClientError::ApiError(error_text, status));
        }

        match response.json::<SwapResponse>().await {
            Ok(swap_response) => Ok(swap_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Fetches a swap transaction from Jupiter's `/swap` endpoint.
    ///
    /// # Arguments
    /// * `data` - The [`SwapRequest`]payload.
    ///
    /// # Returns
    /// A `Result` containing the `SwapInstructions`or an error.
    ///
    /// # Example
    /// ```
    /// let payload = SwapRequest::new("YourPubKey...", quote);
    /// let swap_instructions = api.get_swap_instructions(payload).await?;
    /// ```
    pub async fn get_swap_instructions(
        &self,
        data: SwapRequest,
    ) -> Result<SwapInstructions, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/swap-instructions", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(JupiterClientError::ApiError(error_text, status));
        }

        match response.json::<SwapInstructions>().await {
            Ok(swap_instructions) => Ok(swap_instructions),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JupiterClientError {
    #[error("Request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Invalid header value: {0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),

    #[error("API returned error: {0}, Status Code: {1}")]
    ApiError(String, StatusCode),

    #[error("Failed to deserialize response: {0}")]
    DeserializationError(String),
}

// TODO
//
