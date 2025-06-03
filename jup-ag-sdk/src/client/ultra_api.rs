use crate::{
    error::{JupiterClientError, handle_response},
    types::{
        Router, Shield, TokenBalancesResponse, UltraExecuteOrderRequest, UltraExecuteOrderResponse,
        UltraOrderRequest, UltraOrderResponse,
    },
};

use super::JupiterClient;

impl JupiterClient {
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
        let response = match self
            .client
            .get(format!("{}/ultra/v1/order", self.base_url))
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
        let response = match self
            .client
            .post(format!("{}/ultra/v1/execute", self.base_url))
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
        let response = match self
            .client
            .get(format!("{}/ultra/v1/balances/{}", self.base_url, address))
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
    /// let shield_info = client.shield(&mints).await?;
    /// println!("{:#?}", shield_info);
    /// ```
    pub async fn shield(&self, mints: &[String]) -> Result<Shield, JupiterClientError> {
        let query_params = vec![("mints", mints.join(","))];

        let response = match self
            .client
            .get(format!("{}/ultra/v1/shield", self.base_url))
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
        let response = match self
            .client
            .get(format!("{}/ultra/v1/order/routers", self.base_url))
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
