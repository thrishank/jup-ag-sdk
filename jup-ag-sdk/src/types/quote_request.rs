use serde::{Deserialize, Serialize, Serializer};

/// A request struct for fetching a quote from Jupiter's `/quote` endpoint.
///
/// Use `QuoteRequest::new()` and the fluent setters to configure parameters.
///
/// [Official API docs](https://docs.jup.ag/apis/quote)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    /// The mint address of the input token.
    ///
    /// Example: `"So11111111111111111111111111111111111111112"` (SOL)
    pub input_mint: String,

    /// The mint address of the output token.
    ///
    /// Example: `"JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"`
    pub output_mint: String,

    /// The amount to swap (raw, before decimals).
    ///
    /// Meaning depends on `swap_mode` ExactIn default :
    /// - `ExactIn`: amount of input token
    /// - `ExactOut`: amount of output token
    pub amount: u64,

    /// Slippage tolerance in basis points (bps).
    ///
    /// Example: `100` for 1% slippage.
    /// Optional; used only if `dyanmic_slippage` is `false`.
    pub slippage_bps: Option<u16>,

    /// Determines whether the amount is `ExactIn` or `ExactOut`.
    ///
    /// - `ExactIn`: guarantees input amount, computes output.
    /// - `ExactOut`: guarantees output amount, computes input.
    ///
    /// Default: `ExactIn`
    pub swap_mode: Option<QuoteGetSwapModeEnum>,

    /// A list of DEXes to exclusively include in routing.
    ///
    /// Example: `["Orca", "Meteora+DLMM"]`
    /// TODO: add the list of dexes avaliable
    /// refer: https://github.com/Jupiter-DevRel/jup-ag-sdk/blob/main/jup_ag_sdk/models/common/dex_enum.py
    #[serde(serialize_with = "vec_to_comma_string")]
    pub dexes: Option<Vec<String>>,

    /// A list of DEXes to exclude from routing.
    ///
    /// Example: `["Raydium", "Lifinity"]`
    #[serde(serialize_with = "vec_to_comma_string")]
    pub exclude_dexes: Option<Vec<String>>,

    /// If true, restricts intermediate tokens to a stable set.
    ///
    /// Reduces slippage risk. Default: `true`.
    pub restrict_intermediate_tokens: Option<bool>,

    /// If true, only direct (single-hop) routes are allowed.
    ///
    /// May return suboptimal pricing. Default: `false`.
    pub only_direct_routes: Option<bool>,

    /// If true, the quote is prepared for legacy (v0) transactions.
    ///
    /// Use this if you're building a legacy transaction manually.
    pub as_legacy_transaction: Option<bool>,

    /// Platform fee in basis points (bps).
    ///
    /// Used with `feeAccount` in `/swap` to apply affiliate/platform fees.
    pub platform_fee_bps: Option<u64>,

    /// Upper bound on the number of accounts used in the quote.
    ///
    /// Helps with resource budgeting. Default: `64`
    pub max_accounts: Option<u8>,

    /// Enables Jupiter's dynamic slippage estimation.
    ///
    /// If true, overrides `slippage_bps`.
    pub dynamic_slippage: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum QuoteGetSwapModeEnum {
    ExactIn,
    ExactOut,
}

impl QuoteRequest {
    /// Creates a new `QuoteRequest` with the specified input mint, output mint, and amount.
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
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL (9 decimals)
    /// );
    /// assert_eq!(request.input_mint, "So11111111111111111111111111111111111111112");
    /// assert_eq!(request.output_mint, "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN");
    /// assert_eq!(request.amount, 1_000_000_000);
    /// ```
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
            dynamic_slippage: None,
        }
    }

    /// Sets the slippage tolerance in basis points (bps).
    ///
    /// Only used if `dynamic_slippage` is `false`. 100 bps = 1% slippage.
    ///
    /// # Arguments
    /// * `slippage_bps` - Slippage tolerance in basis points (e.g., 50 for 0.5%).
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .slippage_bps(100); // 1% slippage
    /// assert_eq!(request.slippage_bps, Some(100));
    /// ```
    pub fn slippage_bps(mut self, slippage_bps: u16) -> Self {
        self.slippage_bps = Some(slippage_bps);
        self
    }

    /// Sets the swap mode (`ExactIn` or `ExactOut`).
    ///
    /// - `ExactIn`: Specifies input amount, computes output.
    /// - `ExactOut`: Specifies output amount, computes input.
    ///
    /// # Arguments
    /// * `swap_mode` - The swap mode (`QuoteGetSwapModeEnum::ExactIn` or `ExactOut`).
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000
    /// )
    /// .swap_mode(QuoteGetSwapModeEnum::ExactOut); // Want exactly 1 JUP output
    /// assert_eq!(request.swap_mode, Some(QuoteGetSwapModeEnum::ExactOut));
    /// ```
    pub fn swap_mode(mut self, swap_mode: QuoteGetSwapModeEnum) -> Self {
        self.swap_mode = Some(swap_mode);
        self
    }

    /// Sets the list of DEXes to exclusively include in routing.
    ///
    /// Only routes through these DEXes will be considered.
    ///
    /// # Arguments
    /// * `dexes` - A vector of DEX names (e.g., `["Orca", "Meteora+DLMM"]`).
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .dexes(vec!["Orca+V1".to_string(), "Meteora+DLMM".to_string()]);
    /// assert_eq!(request.dexes, Some(vec!["Orca+V1`".to_string(), "Meteora+DLMM".to_string()]));
    /// ```
    /// [list of dexes](https://lite-api.jup.ag/swap/v1/program-id-to-label)
    pub fn dexes(mut self, dexes: Vec<String>) -> Self {
        self.dexes = Some(dexes);
        self
    }

    /// Sets the list of DEXes to exclude from routing.
    ///
    /// Routes will avoid these DEXes.
    ///
    /// # Arguments
    /// * `exclude_dexes` - A vector of DEX names to exclude (e.g., `["Raydium", "Lifinity"]`).
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    /// use quote_request::{QuoteRequest, QuoteRequestBuilder};
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .exclude_dexes(vec!["Guacswap".to_string(), "Lifinity".to_string()]);
    /// assert_eq!(request.exclude_dexes, Some(vec!["Guacswap".to_string(), "Lifinity".to_string()]));
    /// ```
    /// [list of dexes](https://lite-api.jup.ag/swap/v1/program-id-to-label)
    pub fn exclude_dexes(mut self, exclude_dexes: Vec<String>) -> Self {
        self.exclude_dexes = Some(exclude_dexes);
        self
    }

    /// Sets whether to restrict intermediate tokens to a stable set.
    ///
    /// Reduces slippage risk by limiting intermediate tokens. Default: `true`.
    ///
    /// # Arguments
    /// * `restrict_intermediate_tokens` - Whether to restrict intermediate tokens.
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    /// use quote_request::{QuoteRequest, QuoteRequestBuilder};
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .restrict_intermediate_tokens(true);
    /// assert_eq!(request.restrict_intermediate_tokens, Some(true));
    /// ```
    pub fn restrict_intermediate_tokens(mut self, restrict_intermediate_tokens: bool) -> Self {
        self.restrict_intermediate_tokens = Some(restrict_intermediate_tokens);
        self
    }

    /// Sets whether to allow only direct (single-hop) routes.
    ///
    /// May result in suboptimal pricing. Default: `false`.
    ///
    /// # Arguments
    /// * `only_direct_routes` - Whether to allow only direct routes.
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .only_direct_routes(true);
    /// assert_eq!(request.only_direct_routes, Some(true));
    /// ```
    pub fn only_direct_routes(mut self, only_direct_routes: bool) -> Self {
        self.only_direct_routes = Some(only_direct_routes);
        self
    }

    /// Sets whether the quote is prepared for legacy (v0) transactions.
    ///
    /// Use for manually building legacy transactions.
    ///
    /// # Arguments
    /// * `as_legacy_transaction` - Whether to prepare for legacy transactions.
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .as_legacy_transaction(true);
    /// assert_eq!(request.as_legacy_transaction, Some(true));
    /// ```
    pub fn as_legacy_transaction(mut self, as_legacy_transaction: bool) -> Self {
        self.as_legacy_transaction = Some(as_legacy_transaction);
        self
    }

    /// Sets the platform fee in basis points (bps).
    ///
    /// Used with `feeAccount` in `/swap` for affiliate/platform fees.
    ///
    /// # Arguments
    /// * `platform_fee_bps` - The platform fee in basis points.
    ///
    /// # Returns
    /// The modified `QuoteRequest` for chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112",
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
    ///     1_000_000_000
    /// )
    /// .platform_fee_bps(20); // 0.2% fee
    /// assert_eq!(request.platform_fee_bps, Some(20));
    /// ```
    pub fn platform_fee_bps(mut self, platform_fee_bps: u64) -> Self {
        self.platform_fee_bps = Some(platform_fee_bps);
        self
    }

    /// Sets the upper bound on the number of accounts used in the quote.
    ///
    /// This helps budget resources for the transaction, as some routes may require more accounts.
    /// The default value is typically 64, but you can specify a lower or higher value depending on your needs.
    ///
    /// # Arguments
    /// * `self` - The `QuoteRequest` instance (mutable, consumed by the method).
    /// * `max_accounts` - The maximum number of accounts to use in the quote (e.g., 32 for resource-constrained transactions).
    ///
    /// # Returns
    /// The modified `QuoteRequest` instance for method chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL
    /// )
    /// .max_accounts(32);
    /// assert_eq!(request.max_accounts, Some(32));
    /// ```
    pub fn max_accounts(mut self, max_accounts: u8) -> Self {
        self.max_accounts = Some(max_accounts);
        self
    }

    /// Sets whether to enable Jupiter's dynamic slippage estimation.
    ///
    /// When enabled, this overrides the `slippage_bps` setting, allowing Jupiter to dynamically adjust
    /// slippage based on market conditions. This can reduce the risk of failed transactions due to price volatility.
    ///
    /// # Arguments
    /// * `self` - The `QuoteRequest` instance (mutable, consumed by the method).
    /// * `dynamic_slippage` - Whether to enable dynamic slippage estimation (true to enable, false to use `slippage_bps`).
    ///
    /// # Returns
    /// The modified `QuoteRequest` instance for method chaining.
    ///
    /// # Example
    /// ```
    ///
    /// let request = QuoteRequest::new(
    ///     "So11111111111111111111111111111111111111112", // SOL
    ///     "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
    ///     1_000_000_000 // 1 SOL
    /// )
    /// .dynamic_slippage(true);
    /// assert_eq!(request.dynamic_slippage, Some(true));
    /// ```
    pub fn dynamic_slippage(mut self, dynamic_slippage: bool) -> Self {
        self.dynamic_slippage = Some(dynamic_slippage);
        self
    }
}

pub fn vec_to_comma_string<S>(vec: &Option<Vec<String>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match vec {
        Some(v) => serializer.serialize_str(&v.join(",")),
        None => serializer.serialize_none(),
    }
}
