use jup_ag_sdk::{
    JupiterQuoteApi,
    types::{QuoteRequest, SwapRequest},
};

#[tokio::main]
async fn main() {
    let client = JupiterQuoteApi::new("https://lite-api.jup.ag/swap/v1");

    let quote = QuoteRequest::new(
        "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        "oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp",
        1000000,
    );

    let quote_res = client.get_quote(quote).await.expect("failed to get quote");
    let data = SwapRequest::new("thrbabBvANwvKdV34GdrFUDXB6YMsksdfmiKj2ZUV3m", quote_res);
    let swap = client
        .get_swap_transaction(data)
        .await
        .expect("failed to get swap transaction");

    println!("{:?}", swap);
}
