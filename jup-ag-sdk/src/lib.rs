use error::{JupiterClientError, handle_response};
use reqwest::Client;
use types::{
    QuoteRequest, QuoteResponse, SwapInstructions, SwapRequest, SwapResponse,
    TokenBalancesResponse, UltraExecuteRequest, UltraOrderRequest, UltraOrderResponse,
};

pub mod error;
pub mod types;

/// `JupiterClient` is a client wrapper to interact with the Jupiter Aggregator APIs.
/// It is your gateway to interact with the Jupiter exchange API
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

        let response = match self
            .client
            .get(format!("{}/swap/v1/quote", &self.base_url))
            .headers(headers)
            .query(&params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

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
            .post(format!("{}/swap/v1/swap", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

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
            .post(format!("{}/swap/v1/swap-instructions", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<SwapInstructions>().await {
            Ok(swap_instructions) => Ok(swap_instructions),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    pub async fn get_ultra_order(
        &self,
        params: UltraOrderRequest,
    ) -> Result<UltraOrderResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .get(format!("{}/ultra/v1/order", self.base_url))
            .headers(headers)
            .query(&params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<UltraOrderResponse>().await {
            Ok(ultra_order_response) => Ok(ultra_order_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    pub async fn ultra_excute_transaction(
        &self,
        data: UltraExecuteRequest,
    ) -> Result<UltraOrderResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/ultra/v1/execute", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<UltraOrderResponse>().await {
            Ok(swap_response) => Ok(swap_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for token balances of an account
    pub async fn get_token_balances(
        &self,
        address: &str,
    ) -> Result<TokenBalancesResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .get(format!("{}/ultra/v1/balances/{}", self.base_url, address))
            .headers(headers)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TokenBalancesResponse>().await {
            Ok(token_balances) => Ok(token_balances),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}
