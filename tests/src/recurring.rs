#[cfg(test)]
mod recurring_tests {
    use jup_ag_sdk::types::{
        CreateRecurringOrderRequest, GetRecurringOrders, OrderStatus, RecurringOrderType,
    };

    use crate::common::{SOL_MINT, TEST_USER_PUBKEY, USDC_MINT, create_test_client};

    #[tokio::test]
    async fn test_create_recurring_time_order() {
        let client = create_test_client();
        let data = CreateRecurringOrderRequest::new_time_order(
            TEST_USER_PUBKEY,
            USDC_MINT,
            SOL_MINT,
            1_000_000_000,
            10,
            84600,
        );

        assert_eq!(data.user, TEST_USER_PUBKEY, "User should match");
        assert_eq!(data.input_mint, USDC_MINT, "Input mint should match USDC");
        assert_eq!(data.output_mint, SOL_MINT, "Output mint should match SOL");
        let response = client
            .create_recurring_order(&data)
            .await
            .expect("Failed to create recurring order");

        assert!(
            !response.request_id.is_empty(),
            "Request ID should be present"
        );

        assert!(
            !response.transaction.is_empty(),
            "Unsigned transaction should be present"
        );
    }

    #[tokio::test]
    async fn test_create_recurring_price_order() {
        let client = create_test_client();
        let data = CreateRecurringOrderRequest::new_price_order(
            TEST_USER_PUBKEY,
            USDC_MINT,
            SOL_MINT,
            1_000_000_000,
            100,
            86400,
        );

        assert_eq!(data.user, TEST_USER_PUBKEY, "User should match");
        assert_eq!(data.input_mint, USDC_MINT, "Input mint should match USDC");
        assert_eq!(data.output_mint, SOL_MINT, "Output mint should match SOL");
        let response = client
            .create_recurring_order(&data)
            .await
            .expect("Failed to create recurring order");

        assert!(
            !response.request_id.is_empty(),
            "Request ID should be present"
        );

        assert!(
            !response.transaction.is_empty(),
            "Unsigned transaction should be present"
        );
    }

    #[tokio::test]
    async fn test_get_recurring_orders() {
        let client = create_test_client();
        let req = GetRecurringOrders::new(
            RecurringOrderType::All,
            OrderStatus::History,
            "7EgKcCjBsVjMYv5eZqCe2UZ8xAyCgXzeVZfWwFj3Qiam",
        );

        let history = client
            .get_recurring_orders(&req)
            .await
            .expect("Failed to get recurring orders");

        assert!(history.all.is_some())
    }
}
