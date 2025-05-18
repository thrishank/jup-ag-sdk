#[cfg(test)]
mod tests {

    use jup_ag_sdk::{
        JupiterClient,
        types::{QuoteGetSwapModeEnum, QuoteRequest, SwapRequest},
    };

    #[test]
    fn test_jupiter_client_creation() {
        let base_url = "https://lite-api.jup.ag";

        let client = JupiterClient::new(base_url);

        assert_eq!(client.base_url, base_url);
    }

    #[test]
    fn test_quote_request_builder_methods() {
        let request = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            1_000_000_000,
        )
        .slippage_bps(100)
        .swap_mode(QuoteGetSwapModeEnum::ExactOut)
        .dexes(vec!["Orca".to_string(), "Meteora+DLMM".to_string()])
        .exclude_dexes(vec!["Raydium".to_string()])
        .restrict_intermediate_tokens(false)
        .only_direct_routes(true)
        .as_legacy_transaction(false)
        .platform_fee_bps(10);

        assert_eq!(
            request.input_mint, "So11111111111111111111111111111111111111112",
            "input mint test"
        );
        assert_eq!(
            request.output_mint, "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            "output mint test"
        );
        assert_eq!(request.amount, 1_000_000_000, "amount test");
        assert_eq!(request.slippage_bps, Some(100), "slippage_bps");

        assert_eq!(
            request.dexes,
            Some(vec!["Orca".to_string(), "Meteora+DLMM".to_string()])
        );

        assert_eq!(request.exclude_dexes, Some(vec!["Raydium".to_string()]));

        assert_eq!(request.restrict_intermediate_tokens, Some(false));

        assert_eq!(request.only_direct_routes, Some(true));

        assert_eq!(request.as_legacy_transaction, Some(false));

        assert_eq!(request.platform_fee_bps, Some(10));
    }

    #[tokio::test]
    async fn test_get_quote() {
        let base_url = "https://lite-api.jup.ag";
        let client = JupiterClient::new(base_url);

        let quote = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            1_000_000_000,
        )
        .slippage_bps(100)
        .swap_mode(QuoteGetSwapModeEnum::ExactOut);

        let quote_res = client.get_quote(&quote).await.expect("Failed to get quote");

        assert_eq!(quote_res.input_mint, quote.input_mint);

        assert_eq!(quote_res.output_mint, quote.output_mint);

        assert_eq!(quote_res.out_amount, "1000000000");

        assert_eq!(quote_res.slippage_bps, 100);
    }

    #[tokio::test]
    async fn test_get_quote_http_error() {
        let base_url = "https://lite-api.jup.ag";
        let invalid_client = JupiterClient::new("https://lite-api.jup.ag/invalid");

        let quote = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            1_000_000_000,
        );

        let quote_res = invalid_client.get_quote(&quote).await;

        assert!(quote_res.is_err());

        let valid_client = JupiterClient::new(base_url);

        let quote2 = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "",
            1_000_000_000,
        );

        let quote_res2 = valid_client.get_quote(&quote2).await;

        assert!(quote_res2.is_err());
    }

    #[tokio::test]
    async fn test_swap_request_builder() {
        let base_url = "https://lite-api.jup.ag";
        let client = JupiterClient::new(base_url);

        let quote = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            1_000_000_000,
        )
        .slippage_bps(100)
        .swap_mode(QuoteGetSwapModeEnum::ExactOut);

        let quote_res = client.get_quote(&quote).await.expect("Failed to get quote");

        let swap = SwapRequest::new("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m", quote_res);

        assert_eq!(swap.user_public_key, swap.user_public_key);

        assert_eq!(swap.quote_response.input_mint, quote.input_mint);

        assert_eq!(swap.quote_response.out_amount, "1000000000");
    }

    #[tokio::test]
    async fn test_get_swap() {
        let base_url = "https://lite-api.jup.ag";
        let client = JupiterClient::new(base_url);

        let quote = QuoteRequest::new(
            "So11111111111111111111111111111111111111112",
            "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
            1_000_000_000,
        )
        .slippage_bps(100)
        .swap_mode(QuoteGetSwapModeEnum::ExactOut);

        let quote_res = client.get_quote(&quote).await.expect("Failed to get quote");

        let swap = SwapRequest::new("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m", quote_res);

        let swap_res = client
            .get_swap_transaction(&swap)
            .await
            .expect("failed to get swap transaction");

        assert!(!swap_res.swap_transaction.is_empty());
    }
}
