# Jup-Ag-SDK

A Rust SDK for interacting with Jupiter Exchange APIs.

## Installation

```bash
cargo add jup-ag-sdk
```

```rust
use jup_ag_sdk::{
    JupiterQuoteApi,
    types::{QuoteRequest, SwapRequest},
};

#[tokio::main]
async fn main() {
    // Initialize the client
    let client = JupiterQuoteApi::new("https://lite-api.jup.ag/swap/v1");

    // construct the quote request
    let quote = QuoteRequest::new(
        "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp",
        1000000,
    )
    .swap_mode(jup_ag_sdk::types::QuoteGetSwapModeEnum::ExactOut)
    .slippage_bps(1000);

    // get the quote
    let quote_res = client.get_quote(quote).await.expect("failed to get quote");
    println!("quore response: {:?}", quote_res)

    // construct the swap request
    let payload = SwapRequest::new("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m", quote_res);

    // get the swap transaction
    let swap = client
        .get_swap_transaction(payload)
        .await
        .expect("failed to get swap transaction");

    println!("{:?}", swap);
}
```
