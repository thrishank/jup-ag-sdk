use base64::{Engine, engine::general_purpose::STANDARD};
use bincode::{deserialize, serialize};
use dotenv::dotenv;
use jup_ag_sdk::{
    JupiterClient,
    types::{UltraExecuteOrderRequest, UltraOrderRequest},
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};
use std::env;

pub async fn ultra() {
    // Initialize the client
    let client = JupiterClient::new("https://lite-api.jup.ag");

    // Create an ultra order request
    let ultra = UltraOrderRequest::new(
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "So11111111111111111111111111111111111111112",
        1_000_000,
    )
    .add_taker("EXBdeRCdiNChKyD7akt64n9HgSXEpUtpPEhmbnm4L6iH");

    // Fetch ultra order
    let ultra_res = client
        .get_ultra_order(&ultra)
        .await
        .expect("Failed to get ultra order");

    // Decode base64 transaction
    let swap_tx_bytes = STANDARD
        .decode(ultra_res.transaction.expect("no transaction"))
        .expect("Failed to decode base64 transaction");

    // Deserialize transaction and sign it
    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();

    dotenv().ok();

    let key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set in .env");

    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    let signature = keypair.sign_message(&message);

    if tx.signatures.is_empty() {
        // If no signatures array exists (unlikely with Jupiter)
        tx.signatures.push(signature);
    } else {
        // Replace the first signature (fee payer)
        tx.signatures[0] = signature;
    };
    // Serialize and base64 encode the signed transaction
    let signed_tx_bytes = serialize(&tx).unwrap();
    let base64_signed_tx = STANDARD.encode(&signed_tx_bytes);

    // Create execute order request
    let execute = UltraExecuteOrderRequest {
        signed_transaction: base64_signed_tx,
        request_id: ultra_res.request_id,
    };

    // Execute the transaction
    client
        .ultra_execute_order(&execute)
        .await
        .expect("Failed to execute transaction");

    println!("Transaction: {}", signature);
}
