#[cfg(test)]
mod token_tests {
    use jup_ag_sdk::types::TokenPriceRequest;

    use crate::common::{SOL_MINT, USDC_MINT, create_test_client};

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
}
