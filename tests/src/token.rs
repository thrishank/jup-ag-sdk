#[cfg(test)]
mod token_tests {
    use jup_ag_sdk::types::TokenPriceRequest;

    use crate::common::{JUP_MINT, SOL_MINT, USDC_MINT, create_test_client};

    #[tokio::test]
    async fn test_get_token_balances() {
        let client = create_test_client();
        let tokens = client
            .get_token_balances("372sKPyyiwU5zYASHzqvYY48Sv4ihEujfN5rGFKhVQ9j")
            .await
            .expect("failed to get token balances");

        assert_eq!(
            tokens
                .get("2zMMhcVQEXDtdE6vsFS7S7D5oUodfJHE8vd1gnBouauv")
                .expect("pengu token not found")
                .amount,
            516176755.to_string(),
        )
    }

    #[tokio::test]
    async fn test_get_token_prices() {
        let client = create_test_client();
        let token_mints = vec![SOL_MINT.to_string(), USDC_MINT.to_string()];
        let req = TokenPriceRequest::new(&token_mints);

        assert_eq!(req.token_mints.len(), 2, "mints should be 2");
        assert_eq!(req.token_mints[0], SOL_MINT);
        let res = client
            .get_token_price(&req)
            .await
            .expect("failed to get token prices");

        let usdc_price: f64 = res
            .data
            .get(USDC_MINT)
            .expect("usdc price not found")
            .price
            .parse()
            .expect("failed to parse usdc price");

        assert!(
            (0.9..=1.1).contains(&usdc_price),
            "USDC price {} is out of range (0.9 to 1.1)",
            usdc_price
        );

        let req = TokenPriceRequest::new(&token_mints).with_vs_token(SOL_MINT);

        let res = client
            .get_token_price(&req)
            .await
            .expect("failed to get token prices");

        assert_eq!(
            res.data.get(SOL_MINT).expect("sol price not found").price,
            "1"
        );
    }

    #[tokio::test]
    pub async fn test_token_info() {
        let client = create_test_client();

        let info = client
            .get_token_info(JUP_MINT)
            .await
            .expect("failed to get token info");

        assert_eq!(info.decimals, 6, "JUP decimals should be 6");

        assert_eq!(info.symbol, "JUP")
    }

    #[tokio::test]
    pub async fn test_get_market_mints() {
        let client = create_test_client();

        let mints = client
            .get_market_mints("5rCf1DM8LjKTw4YqhnoLcngyZYeNnQqztScTogYHAS6")
            .await
            .expect("failed to get market mints");

        assert_eq!(mints[0], SOL_MINT, "First mint should be SOL");
        assert_eq!(mints[1], USDC_MINT, "Second mint should be USDC");
    }

    #[tokio::test]
    pub async fn test_get_mints_by_tags() {
        let client = create_test_client();

        let tags = vec![String::from("lst")];

        let mints = client
            .get_mints_by_tags(&tags)
            .await
            .expect("failed to get mints by tags");

        assert!(mints.len() > 1000, "there are more that 1000 lst");
    }
}
