use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bincode::{deserialize, serialize};
use dotenv::dotenv;

use jup_ag_sdk::{
    JupiterClient,
    types::{CreateTriggerOrder, ExecuteTriggerOrder, GetTriggerOrders, OrderStatus},
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use std::env;

async fn trigger() {
    let client = JupiterClient::new("https://lite-api.jup.ag");

    // create order parmas
    let data = CreateTriggerOrder::new(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        "372sKPyyiwU5zYASHzqvYY48Sv4ihEujfN5rGFKhVQ9j",
        "372sKPyyiwU5zYASHzqvYY48Sv4ihEujfN5rGFKhVQ9j",
        10_000_000, // swap 10 USDC for 20 JUP
        20_000_000,
    );

    // get the unsigned transaction to create the order
    let create_order = client
        .create_trigger_order(&data)
        .await
        .expect("Failed to create trigger order");

    println!("Create Trigger Order: {:?}", create_order);

    // Load .env variables into std::env
    dotenv().ok();

    // Read the variable
    let key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");

    // if you have the private key in base58 format. Else skip this decode
    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    let swap_tx_bytes = STANDARD
        .decode(create_order.transaction)
        .expect("Failed to decode base64 transaction");

    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();

    // sign the transaction with the keypair
    let signature = keypair.sign_message(&message);

    if tx.signatures.is_empty() {
        tx.signatures.push(signature);
    } else {
        tx.signatures[0] = signature;
    };

    let signed_tx_bytes = serialize(&tx).unwrap();
    let base64_signed_tx = STANDARD.encode(&signed_tx_bytes);

    // here we execute the trigger order. Instead this you can directly send the transaction using a rpc
    let exe = ExecuteTriggerOrder {
        request_id: create_order.request_id,
        signed_transaction: base64_signed_tx,
    };

    let execute = client
        .execute_trigger_order(&exe)
        .await
        .expect("Failed to execute trigger order");

    println!("Execute Trigger Order: {:?}", execute);

    // get trigger orders for user address
    let params = GetTriggerOrders {
        user: "372sKPyyiwU5zYASHzqvYY48Sv4ihEujfN5rGFKhVQ9j".to_string(),
        order_status: OrderStatus::History,
        input_mint: None,
        output_mint: None,
        include_failed_tx: None,
        page: None,
    };

    let orders = client
        .get_trigger_orders(&params)
        .await
        .expect("Failed to get trigger orders");

    println!("Trigger Orders: {:?}", orders);

    // cance a trigger order
    // let cancel = CancelTriggerOrder::new(
    //     "372sKPyyiwU5zYASHzqvYY48Sv4ihEujfN5rGFKhVQ9j",
    //     "HeyWQcYd9t6BFGDfwh3w13F9KmiSNyPJuRPm49kiynFs",
    // );
    //
    // let cancel_order = client
    //     .cancel_trigger_order(&cancel)
    //     .await
    //     .expect("Failed to cancel trigger order");
    //
    // println!("Cancel Trigger Order: {:?}", cancel_order);
    //
    // and then sign the transaction and excute same as the create_order
}
