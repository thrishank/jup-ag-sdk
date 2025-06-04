# Jup-Ag-SDK

> 🚀 A Rust SDK to interact with the [Jupiter Aggregator](https://jup.ag) APIs.  
> Easily fetch token quotes and execute swaps programmatically from your Rust applications.

## Installation 🛠️

```bash
cargo add jup-ag-sdk
```

or Add this to your `Cargo.toml`:

```toml
[dependencies]
jup-ag-sdk = "0.1.5"
```

## Features

- ✅ All Jupiter APIs included – Ultra, Swap, Trigger, Recurring, Token Info
- 🧱 Strongly typed – Full Rust structs for all request/response types
- 🧠 Composable builders – Chainable methods to customize request payloads (e.g. taker, referral, fee, excluded routers)

## Usage 💡

Below is a simple example that shows how to fetch and execute an Ultra order with the SDK. For detailed example checkout [`examples/`](https://github.com/thrishank/jup-ag-sdk/tree/main/examples)

```rust
use jup_ag_sdk::{
    JupiterClient,
    types::{UltraExecuteOrderRequest, UltraOrderRequest},
};

// initalize the client
let client = JupiterClient::new("https://lite-api.jup.ag");

// Create an ultra order request to swap 10 USDC to SOL
let ultra = UltraOrderRequest::new(
    "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
    "So11111111111111111111111111111111111111112",
    10_000_000, // 6 decimals (USDC)
).add_taker("your wallet address");

// fetch the quote
let quote = client.get_ultra_order(&ultra).await
    .expect("Failed to get ultra order");

// sign the transaction. Checkout examples/src/ultra.rs on how to sign the transaction

// execute the signed transaction
let execute = UltraExecuteOrderRequest {
    signed_transaction: base64_signed_tx,
    request_id: quote.request_id,
};

// Execute the transaction
let response = client.ultra_execute_order(&execute).await
    .expect("Failed to execute transaction");

println!("Transaction: {}", response.signature);
```

## Local

```bash
git clone https://github.com/thrishank/jup-ag-sdk
cd jup-ag-sdk
cargo build
```

open to contributions and suggestions.

## License

MIT License
