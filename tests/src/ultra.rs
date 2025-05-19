#[cfg(test)]
mod ultra_tests {
    use jup_ag_sdk::types::UltraOrderRequest;

    use crate::common::{JUP_MINT, SOL_MINT, TEST_AMOUNT, TEST_USER_PUBKEY, create_test_client};

    #[test]
    fn test_ultra_order_request_builder() {
        let order =
            UltraOrderRequest::new(SOL_MINT, JUP_MINT, TEST_AMOUNT).add_taker(TEST_USER_PUBKEY);

        assert_eq!(order.input_mint, SOL_MINT, "input mint should match");
        assert_eq!(order.output_mint, JUP_MINT, "output mint should match");
        assert_eq!(order.amount, TEST_AMOUNT, "amount should match");
        assert_eq!(
            order.taker,
            Some(TEST_USER_PUBKEY.to_string()),
            "taker should match"
        );
    }

    #[tokio::test]
    async fn test_get_ultra_order_successful() {
        let client = create_test_client();

        let order =
            UltraOrderRequest::new(SOL_MINT, JUP_MINT, 10000000).add_taker(TEST_USER_PUBKEY);

        match client.get_ultra_order(&order).await {
            Ok(order_res) => {
                assert_eq!(
                    order_res.input_mint, order.input_mint,
                    "input mint should match"
                );

                assert_eq!(
                    order_res.output_mint, order.output_mint,
                    "output mint should match"
                );

                assert_eq!(
                    order_res.in_amount,
                    order.amount.to_string(),
                    "amount should match"
                );

                assert_eq!(
                    order_res.taker,
                    Some(TEST_USER_PUBKEY.to_string()),
                    "taker should match"
                );
            }
            Err(err) => panic!("get ultra order should succeed, got error: {:?}", err),
        };
    }

    #[tokio::test]
    async fn test_get_ultra_order_with_invalid_params() {
        let client = create_test_client();

        let order = UltraOrderRequest::new(SOL_MINT, JUP_MINT, 10000).add_taker("invalid taker");
        // This account does not have that much SOL

        let res = client.get_ultra_order(&order).await;
        assert!(
            res.is_err(),
            "Order with a invalid taker address value should fail"
        );

        let order = UltraOrderRequest::new(SOL_MINT, "invalid mint", 10000000000);
        let res = client.get_ultra_order(&order).await;
        assert!(
            res.is_err(),
            "Order with a invalid mint address should fail"
        );

        let order = UltraOrderRequest::new(SOL_MINT, JUP_MINT, 10000000000).exclude_routers(vec![
            "metis".to_string(),
            "jupiterz".to_string(),
            "hashflow".to_string(),
            "dflow".to_string(),
            "pyth".to_string(),
            "okx".to_string(),
        ]);

        let res = client.get_ultra_order(&order).await;
        assert!(res.is_err(), "Order with all routers excluded should fail");
    }

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
    async fn test_shield() {
        let client = create_test_client();

        let mints = vec!["EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string()];

        let shield_res = client.shield(&mints).await.expect("Failed to get shield");
        assert_eq!(
            shield_res.warnings.get(&mints[0]).expect("token not found")[0].warning_type,
            "HAS_FREEZE_AUTHORITY"
        );

        assert_eq!(
            shield_res.warnings.get(&mints[0]).expect("token not found")[0].severity,
            "warning"
        );
    }
}
