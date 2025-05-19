use serde::{Deserialize, Serialize};

use super::QuoteGetSwapModeEnum;

/// A response returned by Jupiter’s `/quote` endpoint.
///
/// Includes detailed routing, fee, and token swap info.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    /// The input token mint address.
    pub input_mint: String,

    /// The raw input token amount.
    pub in_amount: String,

    /// The output token mint address.
    pub output_mint: String,

    /// The raw output token amount (excluding slippage or fees).
    pub out_amount: String,

    /// The worst-case output amount after slippage & fees.
    ///
    /// Not used by `/swap`, but useful for displaying expectations.
    pub other_amount_threshold: String,

    /// Indicates the swap mode used (ExactIn or ExactOut).
    pub swap_mode: QuoteGetSwapModeEnum,

    /// The applied slippage in basis points.
    pub slippage_bps: u16,

    /// Platform fee info (if any was applied).
    #[serde(default)]
    pub platform_fee: Option<PlatformFee>,

    /// Estimated price impact as a percentage string.
    pub price_impact_pct: String,

    /// The detailed route plan (possibly multiple hops).
    pub route_plan: Vec<RoutePlanItem>,

    /// Optional: A diagnostic or scoring report from Jupiter.
    #[serde(default)]
    pub score_report: Option<serde_json::Value>,

    /// The Solana slot number used for this quote.
    pub context_slot: u64,

    /// Total time taken by Jupiter to generate this quote (in seconds).
    pub time_taken: f64,

    /// Optional: Estimated USD value of the swap.
    #[serde(default)]
    pub swap_usd_value: Option<String>,

    /// Optional: Whether a simpler route (e.g. 1-hop) was used.
    #[serde(default)]
    pub simpler_route_used: Option<bool>,

    /// Optional: Reliability report about the AMMs used in routing.
    #[serde(default)]
    pub most_reliable_amms_quote_report: Option<MostReliableAmmsQuoteReport>,

    /// Optional: Slippage estimated by Jupiter’s internal engine.
    #[serde(default)]
    pub use_incurred_slippage_for_quoting: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanItem {
    pub swap_info: SwapInfo,
    pub percent: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: String,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MostReliableAmmsQuoteReport {
    pub info: std::collections::HashMap<String, String>,
}
