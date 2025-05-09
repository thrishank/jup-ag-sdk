use reqwest::Client;
use std::collections::HashMap;
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
    ) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let mut query_params = HashMap::new();
        query_params.insert("inputMint", params.input_mint.to_string());
        query_params.insert("outputMint", params.output_mint.to_string());
        query_params.insert("amount", params.amount.to_string());

        if let Some(slippage_bps) = params.slippage_bps {
            query_params.insert("slippageBps", slippage_bps.to_string());
        }

        if let Some(swap_mode) = params.swap_mode {
            let mode = match swap_mode {
                types::QuoteGetSwapModeEnum::ExactIn => "ExactIn",
                types::QuoteGetSwapModeEnum::ExactOut => "ExactOut",
            };
            query_params.insert("swapMode", mode.to_string());
        }

        if let Some(dexes) = params.dexes {
            query_params.insert("dexes", dexes.join(","));
        }

        if let Some(exclude_dexes) = params.exclude_dexes {
            query_params.insert("excludeDexes", exclude_dexes.join(","));
        }

        if let Some(restrict_intermediate_tokens) = params.restrict_intermediate_tokens {
            query_params.insert(
                "restrictIntermediateTokens",
                restrict_intermediate_tokens.to_string(),
            );
        }

        if let Some(only_direct) = params.only_direct_routes {
            query_params.insert("onlyDirectRoutes", only_direct.to_string());
        }

        if let Some(legacy) = params.as_legacy_transaction {
            query_params.insert("asLegacyTransaction", legacy.to_string());
        }

        if let Some(fee_bps) = params.platform_fee_bps {
            query_params.insert("platformFeeBps", fee_bps.to_string());
        }

        if let Some(max_accounts) = params.max_accounts {
            query_params.insert("maxAccounts", max_accounts.to_string());
        }

        if let Some(slippage) = params.dynamic_slippage {
            query_params.insert("autoSlippage", slippage.to_string());
        }

        let response = match self
            .client
            .get(format!("{}/quote", &self.base_url))
            .headers(headers)
            .query(&query_params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(format!("Error fetching quote: {}", e).into()),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(format!("API returned error status: {} - {}", status, error_text).into());
        }

        match response.json::<QuoteResponse>().await {
            Ok(quote_response) => Ok(quote_response),
            Err(e) => Err(format!("Failed to parse JSON response: {}", e).into()),
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
    ) -> Result<SwapResponse, Box<dyn std::error::Error>> {
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
            Err(e) => return Err(format!("Error fetching swap transaction: {}", e).into()),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(format!("API returned error status: {} - {}", status, error_text).into());
        }

        match response.json::<SwapResponse>().await {
            Ok(swap_response) => Ok(swap_response),
            Err(e) => Err(format!("Failed to parse JSON response: {}", e).into()),
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
    ) -> Result<SwapInstructions, Box<dyn std::error::Error>> {
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
            Err(e) => return Err(format!("Error fetching swap instructions: {}", e).into()),
        };

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to get error details".to_string());
            return Err(format!("API returned error status: {} - {}", status, error_text).into());
        }

        match response.json::<SwapInstructions>().await {
            Ok(swap_instructions) => Ok(swap_instructions),
            Err(e) => Err(format!("Failed to parse JSON response: {}", e).into()),
        }
    }
}
