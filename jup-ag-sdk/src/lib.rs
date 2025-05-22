use error::{JupiterClientError, handle_response};
use reqwest::Client;
use types::{
    QuoteRequest, QuoteResponse, Router, Shield, SwapInstructions, SwapRequest, SwapResponse,
    TokenBalancesResponse, UltraExecuteOrderRequest, UltraExecuteOrderResponse, UltraOrderRequest,
    UltraOrderResponse,
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
            // TODO: Add Api key here
            // make the base_url default
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
        params: &QuoteRequest,
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
        data: &SwapRequest,
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
        data: &SwapRequest,
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

    /// Fetches a swap order from Jupiter's Ultra API based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - An [`UltraOrderRequest`] with fields like input/output mint, amount, taker, and more .
    ///
    /// # Returns
    ///
    /// * `Ok(UltraOrderResponse)` on success.
    /// * `Err` if the request fails or response can't be deserialized.
    ///
    /// # Jupiter API Reference
    ///
    /// - [Ultra Order Endpoint](https://dev.jup.ag/docs/api/ultra-api/order)
    ///
    /// # Example
    ///
    /// ```
    /// let req = UltraOrderRequest::new("inputMint", "outputMint", 1_000_000_000);
    /// let order = api.get_ultra_order(&req).await?;
    /// ```
    pub async fn get_ultra_order(
        &self,
        params: &UltraOrderRequest,
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

    /// Executes a signed swap order using Jupiter's Ultra API.
    ///
    /// # Arguments
    ///
    /// * `data` - An [`UltraExecuteRequest`] containing the signed transaction and request ID.
    ///
    /// # Returns
    ///
    /// * `Ok(UltraExecuteOrderResponse)` on success.
    /// * `Err` if the request or deserialization fails.
    ///
    /// # Jupiter API Reference
    ///
    /// - [Execute Order Endpoint](https://dev.jup.ag/docs/api/ultra-api/execute)
    ///
    /// # Example
    ///
    /// ```
    /// let req = UltraExecuteOrderRequest::new(signed_tx, request_id);
    /// let res = api.ultra_execute_order(&req).await?;
    /// ```
    pub async fn ultra_execute_order(
        &self,
        data: &UltraExecuteOrderRequest,
    ) -> Result<UltraExecuteOrderResponse, JupiterClientError> {
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

        match response.json::<UltraExecuteOrderResponse>().await {
            Ok(swap_response) => Ok(swap_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Fetches token balances for a given wallet address using Jupiter's Ultra API.
    ///
    /// # Arguments
    ///
    /// * `address` - The wallet address to fetch token balances for.
    ///
    /// # Returns
    ///
    /// * `Ok(TokenBalancesResponse)` containing token balances.
    /// * `Err` if the request or deserialization fails.
    ///
    /// # Jupiter API Reference
    ///
    /// - [Balances Endpoint](https://dev.jup.ag/docs/api/ultra-api/balances)
    ///
    /// # Example
    ///
    /// ```
    /// let balances = api.get_token_balances("3X2LFoTQecbpqCR7G5tL1kczqBKurjKPHhKSZrJ4wgWc").await?;
    /// println!("{:?}", balances.get("SOL"));
    /// println!("{:?" balances.get("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN")); // JUP
    /// ```
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

    /// Fetches token safety information for given mints using Jupiter's Ultra Shield API.
    ///
    /// This is useful for identifying malicious or suspicious tokens before executing a swap.
    ///
    /// # Arguments
    ///
    /// * `mints` - A slice of mint addresses (`&[String]`) to inspect.
    ///
    /// # Returns
    ///
    /// * `Ok(Shield)` containing token safety metadata.
    /// * `Err` if the request or deserialization fails.
    ///
    /// # Jupiter API Reference
    ///
    /// - [Shield Endpoint](https://dev.jup.ag/docs/api/ultra-api/shield)
    ///
    /// # Example
    ///
    /// ```
    /// let mints = vec![
    ///     "So11111111111111111111111111111111111111112".to_string(),
    ///     "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
    /// ];
    /// let shield_info = api.shield(&mints).await?;
    /// println!("{:#?}", shield_info);
    /// ```
    pub async fn shield(&self, mints: &[String]) -> Result<Shield, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let query_params = vec![("mints", mints.join(","))];

        let response = match self
            .client
            .get(format!("{}/ultra/v1/shield", self.base_url))
            .headers(headers)
            .query(&query_params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Shield>().await {
            Ok(token_balances) => Ok(token_balances),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for the list of routers available in the routing engine of Ultra, which is Juno
    pub async fn routers(&self) -> Result<Vec<Router>, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .get(format!("{}/ultra/v1/order/routers", self.base_url))
            .headers(headers)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        response
            .json::<Vec<Router>>()
            .await
            .map_err(|e| JupiterClientError::DeserializationError(e.to_string()))
    }
}
