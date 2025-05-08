use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
    pub slippage_bps: Option<u16>,
    pub swap_mode: Option<QuoteGetSwapModeEnum>,

    pub dexes: Option<Vec<String>>,
    pub exclude_dexes: Option<Vec<String>>,
    pub restrict_intermediate_tokens: Option<bool>,
    pub only_direct_routes: Option<bool>,
    pub as_legacy_transaction: Option<bool>,
    pub platform_fee_bps: Option<u64>,
    pub max_accounts: Option<u64>,
    pub dyanmic_slippage: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QuoteGetSwapModeEnum {
    ExactIn,
    ExactOut,
}

impl QuoteRequest {
    pub fn new(input_mint: &str, output_mint: &str, amount: u64) -> Self {
        Self {
            input_mint: input_mint.to_string(),
            output_mint: output_mint.to_string(),
            amount,
            slippage_bps: None,
            swap_mode: None,
            dexes: None,
            exclude_dexes: None,
            restrict_intermediate_tokens: None,
            only_direct_routes: None,
            as_legacy_transaction: None,
            platform_fee_bps: None,
            max_accounts: None,
            dyanmic_slippage: None,
        }
    }

    pub fn slippage_bps(mut self, slippage_bps: u16) -> Self {
        self.slippage_bps = Some(slippage_bps);
        self
    }

    pub fn swap_mode(mut self, swap_mode: QuoteGetSwapModeEnum) -> Self {
        self.swap_mode = Some(swap_mode);
        self
    }

    pub fn dexes(mut self, dexes: Vec<String>) -> Self {
        self.dexes = Some(dexes);
        self
    }

    pub fn exclude_dexes(mut self, exclude_dexes: Vec<String>) -> Self {
        self.exclude_dexes = Some(exclude_dexes);
        self
    }

    pub fn restrict_intermediate_tokens(mut self, restrict_intermediate_tokens: bool) -> Self {
        self.restrict_intermediate_tokens = Some(restrict_intermediate_tokens);
        self
    }

    pub fn only_direct_routes(mut self, only_direct_routes: bool) -> Self {
        self.only_direct_routes = Some(only_direct_routes);
        self
    }

    pub fn as_legacy_transaction(mut self, as_legacy_transaction: bool) -> Self {
        self.as_legacy_transaction = Some(as_legacy_transaction);
        self
    }

    pub fn platform_fee_bps(mut self, platform_fee_bps: u64) -> Self {
        self.platform_fee_bps = Some(platform_fee_bps);
        self
    }

    pub fn max_accounts(mut self, max_accounts: u64) -> Self {
        self.max_accounts = Some(max_accounts);
        self
    }

    pub fn dyanmic_slippage(mut self, dyanmic_slippage: bool) -> Self {
        self.dyanmic_slippage = Some(dyanmic_slippage);
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    pub input_mint: String,
    pub in_amount: String,
    pub output_mint: String,
    pub out_amount: String,

    pub other_amount_threshold: String,
    pub swap_mode: QuoteGetSwapModeEnum,
    pub slippage_bps: i32,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: String,

    pub route_plan: Vec<RoutePlanItem>,
    #[serde(default)]
    pub score_report: Option<serde_json::Value>,
    pub context_slot: u64,
    pub time_taken: f64,
    #[serde(default)]
    pub swap_usd_value: Option<String>,
    #[serde(default)]
    pub simpler_route_used: Option<bool>,
    #[serde(default)]
    pub most_reliable_amms_quote_report: Option<MostReliableAmmsQuoteReport>,
    #[serde(default)]
    pub use_incurred_slippage_for_quoting: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFee {
    pub amount: String,
    pub fee_bps: i32,
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapRequest {
    pub user_public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_and_unwrap_sol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_shared_accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prioritization_fee_lamports: Option<PrioritizationFeeLamports>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_legacy_transaction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_token_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_compute_unit_limit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_user_account_rpc_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dyanmic_slippage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_price_micro_lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockhash_slots_to_expiry: Option<u64>,
    pub quote_response: QuoteResponse,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizationFeeLamports {
    pub jito_tip_lamports: Option<u64>,
    pub priority_level_with_max_lamports: PriorityLevelWithMaxLamports,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriorityLevelWithMaxLamports {
    pub max_lamports: u32,
    pub priority_level: PriorityLevel,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PriorityLevel {
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResponse {
    pub swap_transaction: String,
    pub last_valid_block_height: u64,
    pub prioritization_fee_lamports: u64,
}

impl SwapRequest {
    pub fn new(input_wallet: &str, quote: QuoteResponse) -> Self {
        Self {
            user_public_key: input_wallet.to_string(),
            wrap_and_unwrap_sol: None,
            use_shared_accounts: None,
            fee_account: None,
            tracking_account: None,
            prioritization_fee_lamports: None,
            as_legacy_transaction: None,
            destination_token_account: None,
            dynamic_compute_unit_limit: None,
            skip_user_account_rpc_calls: None,
            dyanmic_slippage: None,
            compute_unit_price_micro_lamports: None,
            blockhash_slots_to_expiry: None,
            quote_response: quote,
        }
    }
}
