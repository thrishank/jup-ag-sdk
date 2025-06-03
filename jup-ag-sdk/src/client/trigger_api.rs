use crate::{
    JupiterClientError,
    error::handle_response,
    types::{
        CancelTriggerOrder, CancelTriggerOrders, CreateTriggerOrder, ExecuteTriggerOrder,
        GetTriggerOrders, OrderResponse, TriggerResponse,
    },
};

use super::JupiterClient;

impl JupiterClient {
    /// Creates a new trigger order on Jupiter
    ///
    /// # Arguments
    /// * `data` - `&CreateTriggerOrder` - The trigger order creation parameters
    ///
    /// # Returns
    /// * `Result<TriggerResponse, JupiterClientError>` - Success returns TriggerResponse with:
    ///   - `request_id: String` - Required to make a request to /execute
    ///   - `transaction: String` - Unsigned base-64 encoded transaction
    ///   - `order: String` - Base-58 account which is the Trigger Order account
    ///   - `code: u8` - Response code
    ///
    /// # Example
    /// ```rust
    /// use jupiter_client::types::CreateTriggerOrder;
    ///
    /// let create_order = CreateTriggerOrder::new(
    ///     "So11111111111111111111111111111111111111112", // SOL mint
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP mint
    ///     "YourMakerWalletAddress...",
    ///     "YourPayerWalletAddress...",
    ///     1000000000, // 1 SOL (in lamports)
    ///     400000000,  // 400 JUP (in smallest unit)
    /// )
    /// .slippage_bps("50") // 0.5% slippage
    /// .expired_at("1704067200"); // Unix timestamp
    ///
    /// let response = client.create_trigger_order(&create_order).await?;
    /// println!("Order created with ID: {}", response.request_id);
    /// ```
    pub async fn create_trigger_order(
        &self,
        data: &CreateTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/trigger/v1/createOrder", self.base_url))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TriggerResponse>().await {
            Ok(create_order_response) => Ok(create_order_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Executes a trigger(create, cancel) order by submitting the signed transaction
    ///
    /// # Arguments
    /// * `data` - `&ExecuteTriggerOrder` - Contains:
    ///   - `request_id: String` - The request ID from create_trigger_order response
    ///   - `signed_transaction: String` - The base-58 signed transaction
    ///
    /// # Returns
    /// * `Result<TriggerResponse, JupiterClientError>` - Success returns TriggerResponse with execution details
    ///
    /// # Example
    /// ```rust
    /// use jupiter_client::types::ExecuteTriggerOrder;
    ///
    /// // Execute the order
    /// let execute_order = ExecuteTriggerOrder::new(
    ///     &create_response.request_id, // found in the response of create_trigger_order, cancel_order_response
    ///     &signed_tx
    /// );
    ///
    /// let response = client.execute_trigger_order(&execute_order).await?;
    /// println!("Order executed successfully");
    /// ```
    pub async fn execute_trigger_order(
        &self,
        data: &ExecuteTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/trigger/v1/execute", self.base_url))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TriggerResponse>().await {
            Ok(execute_order_response) => Ok(execute_order_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for a base64-encoded unsigned trigger order cancellation transaction
    /// Sign the transaction then call the execute_trigger_order function
    ///
    /// # Arguments
    /// * `data` - `&CancelTriggerOrder` - Contains:
    ///   - `maker: String` - Maker wallet address
    ///   - `order: String` - Base-58 account which is the Trigger Order account
    ///   - `compute_unit_price: Option<String>` - Priority fee in microlamports (optional)
    ///
    /// # Returns
    /// * `Result<TriggerResponse, JupiterClientError>` - Returns unsigned cancellation transaction to be signed and executed
    ///
    /// # Example
    /// ```rust
    /// use jupiter_client::types::CancelTriggerOrder;
    ///
    /// let cancel_order = CancelTriggerOrder::new(
    ///     "YourMakerWalletAddress...",
    ///     "TriggerOrderAccountAddress..."
    /// );
    ///
    /// // Get the unsigned cancellation transaction
    /// let cancel_response = client.cancel_trigger_order(&cancel_order).await?;
    /// ```
    pub async fn cancel_trigger_order(
        &self,
        data: &CancelTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/trigger/v1/cancelOrder", self.base_url))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TriggerResponse>().await {
            Ok(cancel_order_response) => Ok(cancel_order_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Cancels multiple trigger orders in a single transaction
    ///
    /// # Arguments
    /// * `data` - `&CancelTriggerOrders` - Contains:
    ///   - `maker: String` - Maker wallet address
    ///   - `order: Vec<String>` - Vector of Base-58 trigger order account addresses
    ///   - `compute_unit_price: Option<String>` - Priority fee in microlamports (optional)
    ///
    /// # Returns
    /// * `Result<TriggerResponse, JupiterClientError>` - Returns unsigned batch cancellation transaction
    ///
    /// # Example
    /// ```rust
    /// use jupiter_client::types::CancelTriggerOrders;
    ///
    /// let cancel_orders = CancelTriggerOrders {
    ///     maker: "YourMakerWalletAddress...".to_string(),
    ///     order: vec![
    ///         "TriggerOrderAccount1...".to_string(),
    ///         "TriggerOrderAccount2...".to_string(),
    ///         "TriggerOrderAccount3...".to_string(),
    ///     ],
    ///     compute_unit_price: Some("1000".to_string()), // 1000 microlamports
    /// };
    ///
    /// // Get unsigned batch cancellation transaction
    /// let cancel_response = client.cancel_trigger_orders(&cancel_orders).await?;
    /// ```
    pub async fn cancel_trigger_orders(
        &self,
        data: &CancelTriggerOrders,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/trigger/v1/cancelOrders", self.base_url))
            .json(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TriggerResponse>().await {
            Ok(cancel_order_response) => Ok(cancel_order_response),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    pub async fn get_trigger_orders(
        &self,
        data: &GetTriggerOrders,
    ) -> Result<OrderResponse, JupiterClientError> {
        let response = match self
            .client
            .get(format!("{}/trigger/v1/getTriggerOrders", self.base_url))
            .query(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<OrderResponse>().await {
            Ok(orders) => Ok(orders),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}
