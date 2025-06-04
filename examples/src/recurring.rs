use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bincode::{deserialize, serialize};
use dotenv::dotenv;
use jup_ag_sdk::{
    JupiterClient,
    types::{CreateRecurringOrderRequest, ExecuteRecurringRequest},
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use std::env;

async fn recurring() {
    let client = JupiterClient::new("https://lite-api.jup.ag");
    // create a recurring time based order to swap 100 USDC for SOL every 10 days, with a duration
    // of 84600 seconds (1 day)
    let data = CreateRecurringOrderRequest::new_time_order(
        "your wallet address",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "So11111111111111111111111111111111111111112",
        1_000_000_000,
        10,
        84600,
    );

    let response = client
        .create_recurring_order(&data)
        .await
        .expect("Failed to create recurring order");

    dotenv().ok();

    let key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");

    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    let swap_tx_bytes = STANDARD
        .decode(response.transaction)
        .expect("Failed to decode base64 transaction");

    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();

    // sign the transaction with the keypair
    let signature = keypair.sign_message(&message);

    tx.signatures[0] = signature;

    let signed_tx_bytes = serialize(&tx).unwrap();
    let base64_signed_tx = STANDARD.encode(&signed_tx_bytes);

    let execute = ExecuteRecurringRequest {
        request_id: response.request_id,
        signed_transaction: base64_signed_tx,
    };

    client
        .execute_recurring_order(&execute)
        .await
        .expect("Failed to execute recurring order");

    println!("{}", signature)
}
