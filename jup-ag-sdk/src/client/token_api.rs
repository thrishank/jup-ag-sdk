use super::JupiterClient;
use crate::{
    error::{JupiterClientError, handle_response},
    types::{TokenPriceRequest, TokenPriceResponse},
};

impl JupiterClient {
    /// Returns prices of specified tokens.
    /// ```
    /// let client = JupiterClient::new("https://lite-api.jup.ag")
    ///
    /// let token_mints = vec![
    ///     "So11111111111111111111111111111111111111112".to_string(),
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN".to_string(),
    ///  ];
    /// let params = TokenPriceRequest::new(&token_mints)
    ///     .with_vs_token("So11111111111111111111111111111111111111112"); // default is USDC
    ///
    /// let price = client.get_token_price(&params).await
    ///     .expect("Failed to get token price");
    //
    ///  let sol_price = price.data.get(token_mints[0].as_str())
    ///     .expect("SOL price not found");
    ///
    /// println!("1 SOL price in SOL: {}", sol_price.price);
    //
    /// let jup_price = price.data.get(token_mints[1].as_str())
    ///     .expect("Jup Token price not found");
    ///
    /// println!("1 JUP price in SOL:  {}", jup_price.price);
    ///  ```
    pub async fn get_token_price(
        &self,
        params: &TokenPriceRequest,
    ) -> Result<TokenPriceResponse, JupiterClientError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let response = match self
            .client
            .get(format!("{}/price/v2", self.base_url))
            .headers(headers)
            .query(&params)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TokenPriceResponse>().await {
            Ok(token_price) => Ok(token_price),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}
