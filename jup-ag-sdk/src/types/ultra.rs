use serde::{Deserialize, Serialize};

use super::{PlatformFee, QuoteGetSwapModeEnum, RoutePlanItem};

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
    /// let request = UltraOrder::new(
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
        }
    }

    /// add the taker account to the UltraOrder
    ///
    /// # Arguments
    /// * `taker` - Taker wallet address
    ///
    /// # Example
    /// ```
    /// let request = UltraOrder::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// ).add_taker("taker wallet address");
    pub fn add_taker(mut self, taker: &str) -> Self {
        self.taker = Some(taker.to_string());
        self
    }
    // TODO: Add the refreel methods in the struct
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
