# Jup-Ag-SDK

A Rust SDK for interacting with Jupiter Exchange APIs.

## Installation 🛠️

```bash
cargo add jup-ag-sdk
```

or Add this to your `Cargo.toml`:

```toml
[dependencies]
jup-ag-sdk = "0.1.3"
```

## Usage 💡

### Ultra API

```rust
use base64::{Engine, engine::general_purpose};
use bincode::{deserialize, serialize};
use jup_ag_sdk::{
    JupiterClient,
    types::{UltraExecuteOrderRequest, UltraOrderRequest}
};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
}
#[tokio::main]
async fn main() {
    // Initialize the client
    let client = JupiterClient::new("https://lite-api.jup.ag/ultra/v1");

    // Create an ultra order request
    let ultra = UltraOrderRequest::new(
        "So11111111111111111111111111111111111111112",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        1_000_000,
    )
    .add_taker("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m");

    // Fetch ultra order
    let ultra_res = client
        .get_ultra_order(ultra)
        .await
        .expect("Failed to get ultra order");

    // Decode base64 transaction
    let swap_tx_bytes = base64::decode(
        ultra_res.transaction.expect("no transaction"),
    )
    .expect("Failed to decode base64 transaction");

    // Deserialize transaction and sign it
    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();

    let key = "your private key";

    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    let signature = keypair.sign_message(&message);

    // Serialize and base64 encode the signed transaction
    let signed_tx_bytes = serialize(&tx).unwrap();
    let base64_signed_tx = general_purpose::STANDARD.encode(&signed_tx_bytes);

    // Create execute order request
    let exexute = UltraExecuteOrderRequest {
        signed_transaction: base64_signed_tx,
        request_id: ultra_res.request_id,
    };

    // Execute the transaction
    let execute_res = client
        .ultra_execute_order(&exexute)
        .await
        .expect("Failed to execute transaction");

    // Output the result
    println!(
        "Execute Transaction: {:?}",
        execute_res.signature.expect("no signature")
    );
}

```

### Swap API

```rust
use jup_ag_sdk::{
    JupiterClient,
    types::{QuoteRequest, SwapRequest},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
}
use bincode::deserialize;

#[tokio::main]
async fn main() {
    // Initialize the client
    let client = JupiterClient::new("https://lite-api.jup.ag/swap/v1");

    // construct the quote request
    let quote = QuoteRequest::new(
        "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp",
        1000000,
    )
    .swap_mode(jup_ag_sdk::types::QuoteGetSwapModeEnum::ExactOut)
    .slippage_bps(1000);

    // get the quote
    let quote_res = client.get_quote(&quote).await.expect("failed to get quote");
    println!("quore response: {:?}", quote_res);

    // construct the swap request
    let payload = SwapRequest::new("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m", quote_res);

    // get the swap transaction
    let swap_res = client
        .get_swap_transaction(&payload)
        .await
        .expect("failed to get swap transaction");

    let key = ""; // signer private key
    let key_bytes = bs58::decode(key)
        .into_vec()
        .expect("Failed to decode base58 private key");

    let keypair = Keypair::from_bytes(&key_bytes).expect("Failed to create Keypair");

    let rpc_url = "https://mainnet.helius-rpc.com/?api-key=";
    let rpc_client =
        RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());

    let swap_tx_bytes =
        base64::decode(swap_res.swap_transaction).expect("Failed to decode base64 transaction");

    let mut tx: VersionedTransaction = deserialize(&swap_tx_bytes).unwrap();
    let message = tx.message.serialize();
    let signature = keypair.sign_message(&message);

    if tx.signatures.is_empty() {
        // If no signatures array exists (unlikely with Jupiter)
        tx.signatures.push(signature);
    } else {
        // Replace the first signature (fee payer)
        tx.signatures[0] = signature;
    };

    let signature = rpc_client.send_and_confirm_transaction(&tx).unwrap();

    println!("Transaction signature: {}", signature);
}
```

## Local

```bash
git clone https://github.com/thrishank/jup-ag-sdk
cd jup-ag-sdk
cargo build
```

## MIT License
