use super::JupiterClient;
use crate::{
    error::{JupiterClientError, handle_response},
    types::{NewTokens, TokenInfoResponse, TokenPriceRequest, TokenPriceResponse},
};

// TODO: examples for reccuring

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
    ///     .with_vs_token("So11111111111111111111111111111111111111112"); // default is USD
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
        let response = match self
            .client
            .get(format!("{}/price/v2", self.base_url))
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

    /// Returns the specified mint address's token information and metadata.
    ///
    /// ```
    /// let token_info = client.get_token_info("JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN").await;
    ///
    /// println!("Token Name: {}", token_info.name);
    /// println!("Token Info: {:?}", token_info);
    /// ```
    pub async fn get_token_info(
        &self,
        mint_address: &str,
    ) -> Result<TokenInfoResponse, JupiterClientError> {
        let url = format!("{}/tokens/v1/token/{}", self.base_url, mint_address);
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<TokenInfoResponse>().await {
            Ok(token_info) => Ok(token_info),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Returns the mints involved in a market.
    pub async fn get_market_mints(
        &self,
        market_address: &str,
    ) -> Result<Vec<String>, JupiterClientError> {
        let url = format!(
            "{}/tokens/v1/market/{}/mints",
            self.base_url, market_address
        );
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Vec<String>>().await {
            Ok(mints) => Ok(mints),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Returns a list of all mints tradable via Jupiter routing.
    /// This endpoint returns greater than 32MB amount of data. May take a while to complete.
    pub async fn get_tradable_mints(&self) -> Result<Vec<String>, JupiterClientError> {
        let url = format!("{}/tokens/v1/mints/tradable", self.base_url);
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Vec<String>>().await {
            Ok(mints) => Ok(mints),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Returns a list of mints with specified tag(s) along with their metadata.
    /// tags: verified, lst, token-2022, etc
    /// ```
    ///
    /// let tags = vec![String::from("verified")];
    /// let tagged = client
    /// .get_mints_by_tags(&tags)
    ///    .await
    ///    .expect("failed to get mints by tags");
    /// ```
    pub async fn get_mints_by_tags(
        &self,
        tags: &[String],
    ) -> Result<Vec<TokenInfoResponse>, JupiterClientError> {
        let url = format!("{}/tokens/v1/tagged/{}", self.base_url, tags.join(","));
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Vec<TokenInfoResponse>>().await {
            Ok(mints) => Ok(mints),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// get new tokens with metadata, created at timestamp and markets.
    pub async fn get_new_tokens(
        &self,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<NewTokens>, JupiterClientError> {
        let mut url = format!("{}/tokens/v1/new", self.base_url);
        if let Some(l) = limit {
            url.push_str(&format!("?limit={}", l));
        }
        if let Some(o) = offset {
            if url.contains('?') {
                url.push_str(&format!("&offset={}", o));
            } else {
                url.push_str(&format!("?offset={}", o));
            }
        }
        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Vec<NewTokens>>().await {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }

    /// Returns all tokens with all metadata.
    /// Do note that calling this endpoint's resource will return a large payload of 300+MB, which would introduce some latency in the call.
    /// Please use carefully and intentionally, else utilize the other endpoints.
    pub async fn get_all_tokens(&self) -> Result<Vec<TokenInfoResponse>, JupiterClientError> {
        let url = format!("{}/tokens/v1/all", self.base_url);

        let response = match self.client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(JupiterClientError::RequestError(e)),
        };

        let response = handle_response(response).await?;

        match response.json::<Vec<TokenInfoResponse>>().await {
            Ok(tokens) => Ok(tokens),
            Err(e) => Err(JupiterClientError::DeserializationError(e.to_string())),
        }
    }
}
