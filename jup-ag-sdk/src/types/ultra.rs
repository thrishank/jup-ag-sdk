use super::{PlatformFee, QuoteGetSwapModeEnum, RoutePlanItem, vec_to_comma_string};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for a base64-encoded unsigned swap transaction to be used in POST
///
/// [Official API docs](https://dev.jup.ag/docs/api/ultra-api/order)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltraOrderRequest {
    /// The mint address of the input token.
    ///
    /// Example: `"So11111111111111111111111111111111111111112"` (SOL)
    pub input_mint: String,

    /// The mint address of the output token.
    ///
    /// Example: `"JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"`
    pub output_mint: String,

    /// The amount to input token to swap (raw, before decimals).
    pub amount: u64,

    /// The user's wallet address
    ///
    /// Note: If the taker is not provided, there will still be an Order Response with no transaction field.
    pub taker: Option<String>,

    /// The referral account addres
    pub referral_account: Option<String>,

    /// referral fee in basis points (bps)
    ///
    /// Possible values: >= 50 and <= 255
    pub referral_fee: Option<u8>,

    /// A list of Routers to exclude from routing.
    ///
    /// Possible values: `[metis, jupiterz, hashflow, dflow, pyth, okx]`
    #[serde(serialize_with = "vec_to_comma_string")]
    pub exclude_routers: Option<Vec<String>>,
}

impl UltraOrderRequest {
    /// Creates a new `UltraOrder` with the specified input mint, output mint, and amount.
    ///
    /// # Arguments
    /// * `input_mint` - The mint address of the input token (e.g., SOL mint).
    /// * `output_mint` - The mint address of the output token (e.g., JUP mint).
    /// * `amount` - The amount to swap (raw, before decimals). Meaning depends on `swap_mode`.
    ///
    /// # Returns
    /// A new `QuoteRequest` instance with None value for optional fields.
    ///
    /// # Example
    /// ```
    /// let request = UltraOrderRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// );
    pub fn new(input_mint: &str, output_mint: &str, amount: u64) -> Self {
        UltraOrderRequest {
            input_mint: input_mint.to_string(),
            output_mint: output_mint.to_string(),
            amount,
            taker: None,
            referral_account: None,
            referral_fee: None,
            exclude_routers: None,
        }
    }

    /// add the taker account to the UltraOrder
    ///
    /// # Arguments
    /// * `taker` - Taker wallet address
    ///
    /// # Example
    /// ```
    /// let request = UltraOrderRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// ).add_taker("taker wallet address");
    pub fn add_taker(mut self, taker: &str) -> Self {
        self.taker = Some(taker.to_string());
        self
    }

    /// Add the referral account to the UltraOrder
    ///
    /// # Arguments
    /// * `referral_account` - The referral account address
    ///
    /// # Returns
    /// The updated UltraOrderRequest with referral account set
    ///
    /// # Example
    /// ```
    /// let request = UltraOrderRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// ).add_referral_account("referral account address");
    pub fn add_referral_account(mut self, referral_account: &str) -> Self {
        self.referral_account = Some(referral_account.to_string());
        self
    }

    /// Add the referral fee to the UltraOrder
    ///
    /// # Arguments
    /// * `fee` - Referral fee in basis points (bps)
    ///
    /// # Returns
    /// The updated UltraOrderRequest with referral fee set
    ///
    /// # Panics
    /// Panics if fee is less than 50 or greater than 255
    ///
    /// # Example
    /// ```
    /// let request = UltraOrderRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// ).add_referral_fee(100); // 1% fee (100 bps)
    pub fn add_referral_fee(mut self, fee: u8) -> Self {
        assert!(fee >= 50, "Referral fee must be between 50 and 255 bps");
        self.referral_fee = Some(fee);
        self
    }

    /// Sets the list of Routers to exclude from routing.
    ///
    ///
    /// # Arguments
    /// * `exclude_dexes` - A vector of DEX names to exclude (e.g., `[metis, jupiterz, hashflow, dflow, pyth, okx]`).
    ///
    /// # Returns
    /// The modified `UltraOrderRequest` for chaining.
    ///
    /// # Example
    /// ```
    /// let request = UltraOrderRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .exclude_dexes(vec!["okx".to_string(), "pyth".to_string()]);
    /// ```
    pub fn exclude_routers(mut self, exclude_routers: Vec<String>) -> Self {
        self.exclude_routers = Some(exclude_routers);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltraOrderResponse {
    /// The input token mint address.
    pub input_mint: String,

    /// The output token mint address.
    pub output_mint: String,

    /// The raw input token amount.
    pub in_amount: String,

    /// The raw output token amount (excluding slippage or fees).
    pub out_amount: String,

    /// The worst-case output amount after slippage & fees.
    ///
    /// Not used by `/swap`, but useful for displaying expectations.
    pub other_amount_threshold: String,

    /// Indicates the swap mode used (ExactIn or ExactOut).
    pub swap_mode: QuoteGetSwapModeEnum,

    /// The applied slippage in basis points.
    pub slippage_bps: i32,

    /// Estimated price impact as a percentage string.
    pub price_impact_pct: String,

    /// The detailed route plan (possibly multiple hops).
    pub route_plan: Vec<RoutePlanItem>,

    #[serde(default)]
    pub fee_mint: Option<String>,

    pub fee_bps: u8,

    pub prioritization_fee_lamports: u64,

    pub swap_type: SwapType,

    #[serde(default)]
    pub transaction: Option<String>,

    pub gasless: bool,

    pub request_id: String,

    pub total_time: u16,

    #[serde(default)]
    pub taker: Option<String>,

    #[serde(default)]
    pub quote_id: Option<String>,

    #[serde(default)]
    pub maker: Option<String>,

    /// Platform fee info (if any was applied).
    #[serde(default)]
    pub platform_fee: Option<PlatformFee>,

    #[serde(default)]
    pub expire_at: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SwapType {
    Aggregator,
    Rfq,
    Hashflow,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltraExecuteOrderRequest {
    /// The signed transaction to execute
    pub signed_transaction: String,

    /// Found in response of /order
    pub request_id: String,
}

impl UltraExecuteOrderRequest {
    // function to construct a new UltraExecuteOrderRequest
    //
    // # Arguments
    // * signed_transaction - The signed transaction to execute
    // * request_id - The request ID from the order response
    pub fn new(signed_transaction: &str, request_id: &str) -> Self {
        UltraExecuteOrderRequest {
            signed_transaction: signed_transaction.to_string(),
            request_id: request_id.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UltraExecuteOrderResponse {
    pub status: Status,

    #[serde(default)]
    pub signature: Option<String>,

    #[serde(default)]
    pub slot: Option<String>,

    #[serde(default)]
    pub error: Option<String>,

    pub code: u32,

    #[serde(default)]
    pub total_input_amount: Option<String>,

    #[serde(default)]
    pub total_output_amount: Option<String>,

    #[serde(default)]
    pub input_amount_result: Option<String>,

    #[serde(default)]
    pub output_amount_result: Option<String>,

    #[serde(default)]
    pub swap_events: Option<Vec<SwapEvent>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Success,
    Failed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    pub input_mint: Option<String>,
    pub input_amount: Option<String>,
    pub output_mint: Option<String>,
    pub output_amount: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub amount: String,
    pub ui_amount: f64,
    pub slot: u64,
    pub is_frozen: bool,
}

pub type TokenBalancesResponse = HashMap<String, TokenBalance>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Shield {
    pub warnings: HashMap<String, Vec<Warning>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    #[serde(rename = "type")]
    pub warning_type: String,
    pub message: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Router {
    pub id: String,
    pub name: String,
    pub icon: String,
}
