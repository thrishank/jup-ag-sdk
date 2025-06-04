use crate::{
    JupiterClientError,
    error::handle_response,
    types::{
        CancelRecurringOrderRequest, CreateRecurringOrderRequest, ExecuteRecurringRequest,
        ExecuteRecurringResponse, GetRecurringOrders, PriceDeposit, PriceWithdraw, RecurringOrders,
        RecurringResponse,
    },
};

use super::JupiterClient;

impl JupiterClient {
    /// Sends a request to create a new recurring order.
    ///
    /// Returns a base64-encoded unsigned transaction to be signed and a request _id.
    pub async fn create_recurring_order(
        &self,
        data: &CreateRecurringOrderRequest,
    ) -> Result<RecurringResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/recurring/v1/createOrder", self.base_url))
            .json(data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<RecurringResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for a base64-encoded unsigned recurring order cancellation transaction
    pub async fn cancel_recurring_order(
        &self,
        data: &CancelRecurringOrderRequest,
    ) -> Result<RecurringResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/recurring/v1/cancelOrder", self.base_url))
            .json(data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<RecurringResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for a base64-encoded unsigned price-based recurring order deposit transaction
    pub async fn price_deposit_recurring(
        &self,
        data: &PriceDeposit,
    ) -> Result<RecurringResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/recurring/v1/priceDeposit", self.base_url))
            .json(data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<RecurringResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for a base64-encoded unsigned price-based recurring order withdrawal transaction
    pub async fn price_withdraw_recurring(
        &self,
        data: &PriceWithdraw,
    ) -> Result<RecurringResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/recurring/v1/priceWithdraw", self.base_url))
            .json(data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<RecurringResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// execute a recurring order
    pub async fn execute_recurring_order(
        &self,
        data: &ExecuteRecurringRequest,
    ) -> Result<ExecuteRecurringResponse, JupiterClientError> {
        let response = match self
            .client
            .post(format!("{}/recurring/v1/execute", self.base_url))
            .json(data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<ExecuteRecurringResponse>().await {
            Ok(resp) => Ok(resp),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Request for the active or historical orders associated to the provided account
    pub async fn get_recurring_orders(
        &self,
        data: &GetRecurringOrders,
    ) -> Result<RecurringOrders, JupiterClientError> {
        let response = match self
            .client
            .get(format!("{}/recurring/v1/getRecurringOrders", self.base_url))
            .query(&data)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<RecurringOrders>().await {
            Ok(orders) => Ok(orders),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}
