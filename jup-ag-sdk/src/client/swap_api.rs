use super::JupiterClient;
use crate::{
    error::{JupiterClientError, handle_response},
    types::{QuoteRequest, QuoteResponse, SwapInstructions, SwapRequest, SwapResponse},
};

impl JupiterClient {
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
        let response = match self
            .client
            .get(format!("{}/swap/v1/quote", &self.base_url))
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
        let response = match self
            .client
            .post(format!("{}/swap/v1/swap", self.base_url))
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
        let response = match self
            .client
            .post(format!("{}/swap/v1/swap-instructions", self.base_url))
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
}
