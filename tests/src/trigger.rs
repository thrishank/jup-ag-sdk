#[cfg(test)]
mod ultra_tests {
    use jup_ag_sdk::types::CreateTriggerOrder;

    use crate::common::{SOL_MINT, TEST_USER_PUBKEY, USDC_MINT, create_test_client};

    #[test]
    fn test_trigger_create_order_builder() {
        let create_order = CreateTriggerOrder::new(
            SOL_MINT,
            USDC_MINT,
            TEST_USER_PUBKEY,
            TEST_USER_PUBKEY,
            1_000_000_000,
            200_000_000,
        )
        .expired_at("1748622171");

        assert_eq!(create_order.input_mint, SOL_MINT, "input mint should match");

        assert_eq!(
            create_order.output_mint, USDC_MINT,
            "output mint should match"
        );

        assert_eq!(create_order.payer, TEST_USER_PUBKEY);
        assert_eq!(create_order.maker, TEST_USER_PUBKEY);

        assert_eq!(
            create_order
                .params
                .expired_at
                .expect("experied is None error"),
            "1748622171"
        );
    }

    #[tokio::test]
    async fn test_create_order() {
        let client = create_test_client();

        let params = CreateTriggerOrder::new(
            SOL_MINT,
            USDC_MINT,
            TEST_USER_PUBKEY,
            TEST_USER_PUBKEY,
            1_000_000_000,
            200_000_000,
        )
        .expired_at("1748622171");

        let create_order = client
            .create_trigger_order(&params)
            .await
            .expect("create order failed");

        assert!(
            !create_order.request_id.is_empty(),
            "request_id should not be empty"
        );

        assert!(
            !create_order.transaction.is_empty(),
            "transaction should not be empty"
        );

        assert!(
            !create_order.order.is_empty(),
            "signature should not be empty"
        );
    }
}
