use crate::types::to_comma_string;
use serde::{Deserialize, Serialize};

/// Request for a base64-encoded unsigned trigger order creation transaction
///
/// [Official API docs](https://dev.jup.ag/docs/api/trigger-api/create-order)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTriggerOrder {
    /// The mint address of the input token.
    ///
    /// Example: `"So11111111111111111111111111111111111111112"` (SOL)
    pub input_mint: String,

    /// The mint address of the output token.
    ///
    /// Example: `"JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"`
    pub output_mint: String,

    /// Maker address
    pub maker: String,

    /// fee payer address
    pub payer: String,

    /// making and taking amount inputs
    pub params: Params,

    /// In microlamports, defaults to 95th percentile of priority fees
    /// Default value: auto
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_price: Option<String>,

    /// A token account (via the Referral Program) that will receive the fees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_account: Option<String>,

    /// If either input or output mint is native SOL
    /// Default value: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap_and_unwrap_sol: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    /// Amount of input mint to swap
    pub making_amount: String,

    /// Amount of output mint to receive
    pub taking_amount: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_at: Option<String>,

    /// Amount of slippage the order can be executed with
    /// Default value: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<String>,

    /// Requires the feeAccount parameter, the amount of fees in bps that will be sent to the fee account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_bps: Option<String>,
}

impl CreateTriggerOrder {
    /// Creates a new trigger order with required parameters
    pub fn new(
        input_mint: &str,
        output_mint: &str,
        maker: &str,
        payer: &str,
        making_amount: u64,
        taking_amount: u64,
    ) -> Self {
        Self {
            input_mint: input_mint.to_string(),
            output_mint: output_mint.to_string(),
            maker: maker.to_string(),
            payer: payer.to_string(),
            params: Params::new(making_amount, taking_amount),
            compute_unit_price: None,
            fee_account: None,
            wrap_and_unwrap_sol: None,
        }
    }

    /// Sets the compute unit price in microlamports
    /// Default value: auto
    pub fn compute_unit_price(mut self, price: &str) -> Self {
        self.compute_unit_price = Some(price.to_string());
        self
    }

    /// Sets the fee account for referral program
    pub fn fee_account(mut self, account: &str) -> Self {
        self.fee_account = Some(account.to_string());
        self
    }

    /// Sets whether to wrap and unwrap SOL
    pub fn wrap_and_unwrap_sol(mut self, wrap: bool) -> Self {
        self.wrap_and_unwrap_sol = Some(wrap);
        self
    }

    /// Sets the expiration time for the order
    pub fn expired_at(mut self, expired_at: &str) -> Self {
        self.params.expired_at = Some(expired_at.to_string());
        self
    }

    /// Sets the slippage in basis points
    /// Default value: 0
    pub fn slippage_bps(mut self, slippage: &str) -> Self {
        self.params.slippage_bps = Some(slippage.to_string());
        self
    }

    /// Sets the fee in basis points (requires fee_account to be set)
    pub fn fee_bps(mut self, fee: &str) -> Self {
        self.params.fee_bps = Some(fee.to_string());
        self
    }
}

impl Params {
    /// Creates new parameters with required amounts
    pub fn new(making_amount: u64, taking_amount: u64) -> Self {
        Self {
            making_amount: making_amount.to_string(),
            taking_amount: taking_amount.to_string(),
            expired_at: None,
            slippage_bps: None,
            fee_bps: None,
        }
    }

    /// Sets expiration time (Unix timestamp or relative time)
    pub fn expired_at(mut self, expired_at: &str) -> Self {
        self.expired_at = Some(expired_at.to_string());
        self
    }

    /// Sets slippage tolerance in basis points
    pub fn slippage_bps(mut self, slippage: &str) -> Self {
        self.slippage_bps = Some(slippage.to_string());
        self
    }

    /// Sets fee in basis points
    pub fn fee_bps(mut self, fee: &str) -> Self {
        self.fee_bps = Some(fee.to_string());
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerResponse {
    /// Required to make a request to /execute
    pub request_id: String,

    /// Unsigned base-64 encoded transaction
    pub transaction: String,

    /// Base-58 account which is the Trigger Order account
    pub order: String,

    pub code: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteTriggerOrder {
    /// The request ID  
    pub request_id: String,

    /// The base-58 signed transaction to execute
    pub signed_transaction: String,
}

impl ExecuteTriggerOrder {
    pub fn new(request_id: &str, signed_transaction: &str) -> Self {
        Self {
            request_id: request_id.to_string(),
            signed_transaction: signed_transaction.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTriggerOrder {
    pub maker: String,

    /// Base-58 account which is the Trigger Order account
    pub order: String,

    /// In microlamports, defaults to 95th percentile of priority fees
    /// Default value: auto
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_price: Option<String>,
}

impl CancelTriggerOrder {
    pub fn new(maker: &str, order: &str) -> Self {
        Self {
            maker: maker.to_string(),
            order: order.to_string(),
            compute_unit_price: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelTriggerOrders {
    pub maker: String,

    /// Base-58 account which is the Trigger Order account
    #[serde(serialize_with = "to_comma_string")]
    pub order: Vec<String>,

    /// In microlamports, defaults to 95th percentile of priority fees
    /// Default value: auto
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_unit_price: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTriggerOrders {
    /// user wallet address to retrive orders for
    pub user: String,

    /// Default value: 1
    pub page: Option<String>,

    /// Whether to include failed transactions, expects 'true' or 'false'
    /// Possible values: [true, false]
    pub include_failed_tx: Option<String>,

    /// The status of the orders to return
    /// Possible values: [active, history]
    pub order_status: OrderStatus,

    /// The input mint to filter by
    pub input_mint: Option<String>,

    /// The output mint to filter by
    pub output_mint: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Active,
    History,
}

impl GetTriggerOrders {
    /// Creates a new request to get trigger orders for a user
    pub fn new(user: &str, order_status: OrderStatus) -> Self {
        Self {
            user: user.to_string(),
            page: None,
            include_failed_tx: Some("false".to_string()),
            order_status,
            input_mint: None,
            output_mint: None,
        }
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: &str) -> Self {
        self.page = Some(page.to_string());
        self
    }

    /// Sets whether to include failed transactions
    pub fn include_failed_tx(mut self, include: bool) -> Self {
        self.include_failed_tx = Some(include.to_string());
        self
    }

    /// Sets the order status to filter by
    pub fn order_status(mut self, status: OrderStatus) -> Self {
        self.order_status = status;
        self
    }

    /// Sets the input mint to filter by
    pub fn input_mint(mut self, mint: &str) -> Self {
        self.input_mint = Some(mint.to_string());
        self
    }

    /// Sets the output mint to filter by
    pub fn output_mint(mut self, mint: &str) -> Self {
        self.output_mint = Some(mint.to_string());
        self
    }
}

/// orders associated to the provided user wallet address
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub user: String,
    pub order_status: String,
    pub orders: Vec<Order>,
    pub total_pages: u32,
    pub page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub user_pubkey: String,
    pub order_key: String,
    pub input_mint: String,
    pub output_mint: String,
    pub making_amount: String,
    pub taking_amount: String,
    pub remaining_making_amount: String,
    pub remaining_taking_amount: String,
    pub raw_making_amount: String,
    pub raw_taking_amount: String,
    pub raw_remaining_making_amount: String,
    pub raw_remaining_taking_amount: String,
    pub slippage_bps: String,
    #[serde(default)]
    pub expired_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub status: String,
    pub open_tx: String,
    pub close_tx: String,
    pub program_version: String,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub order_key: String,
    pub keeper: String,
    pub input_mint: String,
    pub output_mint: String,
    pub input_amount: String,
    pub output_amount: String,
    pub raw_input_amount: String,
    pub raw_output_amount: String,
    pub fee_mint: String,
    pub fee_amount: String,
    pub raw_fee_amount: String,
    pub tx_id: String,
    pub confirmed_at: String,
    pub action: String,
    #[serde(default)]
    pub product_meta: Option<serde_json::Value>, // Flexible for null or arbitrary JSON
}
