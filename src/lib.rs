use reqwest::Client;
use std::collections::HashMap;
use types::{QuoteRequest, QuoteResponse, SwapRequest, SwapResponse};
pub mod types;

pub struct JupiterQuoteApi {
    client: Client,
    base_url: String,
}

impl JupiterQuoteApi {
    pub fn new(base_url: &str) -> Self {
        let client = Client::new();
        JupiterQuoteApi {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_quote(
        &self,
        params: QuoteRequest,
    ) -> Result<QuoteResponse, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Accept", "application/json".parse()?);

        let mut query_params = HashMap::new();
        query_params.insert("inputMint", params.input_mint.to_string());
        query_params.insert("outputMint", params.output_mint.to_string());
        query_params.insert("amount", params.amount.to_string());

        if let Some(slippage_bps) = params.slippage_bps {
            query_params.insert("slippageBps", slippage_bps.to_string());
        }

        if let Some(swap_mode) = params.swap_mode {
            let mode = match swap_mode {
                types::QuoteGetSwapModeEnum::ExactIn => "ExactIn",
                types::QuoteGetSwapModeEnum::ExactOut => "ExactOut",
            };
            query_params.insert("swapMode", mode.to_string());
        }

        if let Some(dexes) = params.dexes {
            query_params.insert("dexes", dexes.join(","));
        }

        if let Some(exclude_dexes) = params.exclude_dexes {
            query_params.insert("excludeDexes", exclude_dexes.join(","));
        }

        if let Some(restrict_intermediate_tokens) = params.restrict_intermediate_tokens {
            query_params.insert(
                "restrictIntermediateTokens",
                restrict_intermediate_tokens.to_string(),
            );
        }

        if let Some(only_direct) = params.only_direct_routes {
            query_params.insert("onlyDirectRoutes", only_direct.to_string());
        }

        if let Some(legacy) = params.as_legacy_transaction {
            query_params.insert("asLegacyTransaction", legacy.to_string());
        }

        if let Some(fee_bps) = params.platform_fee_bps {
            query_params.insert("platformFeeBps", fee_bps.to_string());
        }

        if let Some(max_accounts) = params.max_accounts {
            query_params.insert("maxAccounts", max_accounts.to_string());
        }

        if let Some(slippage) = params.dyanmic_slippage {
            query_params.insert("autoSlippage", slippage.to_string());
        }

        let response = self
            .client
            .get(format!("{}/quote", &self.base_url))
            .headers(headers)
            .query(&query_params)
            .send()
            .await?
            .json::<QuoteResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_swap_transaction(
        &self,
        data: SwapRequest,
    ) -> Result<SwapResponse, Box<dyn std::error::Error>> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse()?);
        headers.insert("Accept", "application/json".parse()?);

        // error is in how i am sending the body data
        let response = self
            .client
            .post(format!("{}/swap", self.base_url))
            .headers(headers)
            .json(&data)
            .send()
            .await?
            .json::<SwapResponse>()
            .await?;

        Ok(response)
    }
}
