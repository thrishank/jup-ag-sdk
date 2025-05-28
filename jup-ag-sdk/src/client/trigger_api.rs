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
    pub async fn create_trigger_order(
        &self,
        data: &CreateTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/trigger/v1/createOrder", self.base_url))
            .headers(headers)
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

    pub async fn execute_trigger_order(
        &self,
        data: &ExecuteTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/trigger/v1/execute", self.base_url))
            .headers(headers)
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
    /// sign the transaction then call the execute_trigger_order function
    pub async fn cancel_trigger_order(
        &self,
        data: &CancelTriggerOrder,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/trigger/v1/cancelOrder", self.base_url))
            .headers(headers)
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

    pub async fn cancel_trigger_orders(
        &self,
        data: &CancelTriggerOrders,
    ) -> Result<TriggerResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .post(format!("{}/trigger/v1/cancelOrders", self.base_url))
            .headers(headers)
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
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .get(format!("{}/trigger/v1/getTriggerOrders", self.base_url))
            .headers(headers)
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
