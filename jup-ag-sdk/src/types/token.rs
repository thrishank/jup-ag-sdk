use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenPriceRequest {
    /// Comma separate to pass in multiple
    /// Example: So11111111111111111111111111111111111111112,EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
    #[serde(rename = "ids")]
    #[serde(serialize_with = "vec_to_comma_string")]
    pub token_mints: Vec<String>,

    /// By default, prices are denominated by USD. To denominate price in SOL, use vsToken with SOL mint address
    pub vs_token: Option<String>,

    /// To use, pass in showExtraInfo=true, cannot use vsToken with this parameter
    pub show_extra_info: Option<bool>,
}

impl TokenPriceRequest {
    pub fn new(token_mints: Vec<String>) -> Self {
        Self {
            token_mints,
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

fn vec_to_comma_string<S>(vec: &[String], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&vec.join(","))
}
