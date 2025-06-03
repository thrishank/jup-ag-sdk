use serde::{Deserialize, Serialize};

use super::OrderStatus;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecurringOrderRequest {
    pub user: String,
    pub input_mint: String,
    pub output_mint: String,
    pub params: OrderParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum OrderParams {
    TimeWrapper { time: TimeParams },
    PriceWrapper { price: PriceParams },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeParams {
    pub in_amount: u64,
    pub number_of_orders: u64,
    pub interval: u64,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub start_at: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PriceParams {
    pub deposit_amount: u64,
    pub increment_usdc_value: u64,
    pub interval: u64,
    pub start_at: Option<u64>,
}

impl CreateRecurringOrderRequest {
    pub fn new_time_order(
        user: impl Into<String>,
        input_mint: impl Into<String>,
        output_mint: impl Into<String>,
        in_amount: u64,
        number_of_orders: u64,
        interval: u64,
    ) -> Self {
        let params = TimeParams {
            in_amount,
            number_of_orders,
            interval,
            min_price: None,
            max_price: None,
            start_at: None,
        };
        Self {
            user: user.into(),
            input_mint: input_mint.into(),
            output_mint: output_mint.into(),
            params: OrderParams::TimeWrapper { time: params },
        }
    }

    pub fn new_price_order(
        user: impl Into<String>,
        input_mint: impl Into<String>,
        output_mint: impl Into<String>,
        deposit_amount: u64,
        increment_usdc_value: u64,
        interval: u64,
    ) -> Self {
        let params = PriceParams {
            deposit_amount,
            increment_usdc_value,
            interval,
            start_at: None,
        };

        Self {
            user: user.into(),
            input_mint: input_mint.into(),
            output_mint: output_mint.into(),
            params: OrderParams::PriceWrapper { price: params },
        }
    }

    /// Optional customization for `start_at`, `min_price`, `max_price`
    pub fn with_start_at(mut self, start_at: u64) -> Self {
        match &mut self.params {
            OrderParams::TimeWrapper { time } => time.start_at = Some(start_at),
            OrderParams::PriceWrapper { price } => price.start_at = Some(start_at),
        }
        self
    }

    pub fn with_min_price(mut self, price: f64) -> Self {
        if let OrderParams::TimeWrapper { time } = &mut self.params {
            time.min_price = Some(price);
        }
        self
    }

    pub fn with_max_price(mut self, price: f64) -> Self {
        if let OrderParams::TimeWrapper { time } = &mut self.params {
            time.max_price = Some(price);
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRecurringOrderRequest {
    pub order: String,

    pub recurring_type: RecurringOrderType,

    pub user: String,
}

impl CancelRecurringOrderRequest {
    pub fn new(
        order: impl Into<String>,
        recurring_type: RecurringOrderType,
        user: impl Into<String>,
    ) -> Self {
        Self {
            order: order.into(),
            recurring_type,
            user: user.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecurringOrderType {
    Time,
    Price,
    /// All type is to only be used to get all recurring orders not a actual order type
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDeposit {
    pub amount: u64,

    pub order: String,

    pub user: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceWithdraw {
    /// If no amount is provided, it will withdraw the entire amount
    pub amount: u64,

    pub order: String,

    pub user: String,

    /// Possible values: [In, Out]
    pub input_or_output: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringResponse {
    pub request_id: String,

    /// Unsigned base-64 encoded transaction
    pub transaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRecurringRequest {
    pub request_id: String,

    pub signed_transaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRecurringResponse {
    pub signature: String,

    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRecurringOrders {
    pub recurring_type: RecurringOrderType,
    pub order_status: OrderStatus,
    pub user: String,
    pub page: u64,
    pub mint: Option<String>,
    pub include_failed_tx: bool,
}

impl GetRecurringOrders {
    /// Basic constructor
    pub fn new(
        recurring_type: RecurringOrderType,
        order_status: OrderStatus,
        user: impl Into<String>,
    ) -> Self {
        Self {
            recurring_type,
            order_status,
            user: user.into(),
            page: 1,
            mint: None,
            include_failed_tx: false,
        }
    }

    /// Customize page number
    pub fn with_page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    /// Filter by a specific mint
    pub fn with_mint(mut self, mint: impl Into<String>) -> Self {
        self.mint = Some(mint.into());
        self
    }

    /// Include failed transactions
    pub fn include_failed(mut self) -> Self {
        self.include_failed_tx = true;
        self
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringOrders {
    pub order_status: OrderStatus,
    pub page: u64,
    pub total_pages: u64,
    pub user: String,
    #[serde(default)]
    pub time: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub price: Option<Vec<serde_json::Value>>,
    #[serde(default)]
    pub all: Option<Vec<serde_json::Value>>,
}
