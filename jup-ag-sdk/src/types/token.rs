use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceRequest {
    /// Comma separate to pass in multiple
    /// Example: So11111111111111111111111111111111111111112,EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    #[serde(rename = "ids")]
    #[serde(serialize_with = "to_comma_string")]
    pub token_mints: Vec<String>,

    /// By default, prices are denominated by USD. To denominate price in SOL, use vsToken with SOL mint address
    pub vs_token: Option<String>,

    /// To use, pass in showExtraInfo=true, cannot use vsToken with this parameter
    pub show_extra_info: Option<bool>,
}

impl TokenPriceRequest {
    pub fn new(token_mints: &[String]) -> Self {
        Self {
            token_mints: token_mints.to_vec(),
            vs_token: None,
            show_extra_info: None,
        }
    }

    /// By default, prices are denominated by USD.
    /// For example: To denominate price in SOL, use vsToken with SOL mint address
    pub fn with_vs_token(mut self, vs_token: &str) -> Self {
        self.vs_token = Some(vs_token.to_string());
        self
    }

    /// Boolean flag to show extra info
    pub fn with_show_extra_info(mut self, show_extra_info: bool) -> Self {
        self.show_extra_info = Some(show_extra_info);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    pub id: String,

    #[serde(rename = "type")]
    pub data_type: String,

    pub price: String,

    #[serde(default)]
    pub extra_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceResponse {
    pub data: HashMap<String, TokenPrice>,
    pub time_taken: f64,
}

pub fn to_comma_string<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&vec.join(","))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfoResponse {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub tags: Vec<Option<String>>,
    pub daily_volume: Option<f64>,
    pub created_at: String,
    pub freeze_authority: Option<String>,
    pub mint_authority: Option<String>,
    pub permanent_delegate: Option<String>,
    pub minted_at: Option<String>,
    pub extensions: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTokens {
    pub mint: String,
    pub created_at: String,
    pub metadata_updated_at: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub logo_uri: Option<String>,
    pub known_markets: Vec<String>,
    pub mint_authority: Option<String>,
    pub freeze_authority: Option<String>,
}
