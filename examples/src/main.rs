mod recurring;
mod swap;
mod token;
mod trigger;
mod ultra;

#[tokio::main]
async fn main() {
    // The Jupiter Ultra API is the only API you ever need to experience or build the best trading experience on Solana.
    // Ultra API is the spiritual successor to Swap API, and is much simpler to use than Swap API.
    // If you are first starting out on your Solana development journey, using Ultra API is highly recommended over Swap API.
    // https://dev.jup.ag/docs/ultra-api/
    // ultra::ultra().await;

    // swap::swap().await; // swap tokens using Jupiter swap api
    // swap::swap_with_instructions().await; // build the transaction with Jupiter swap instructions

    // token::token_balances().await; // get token balances using Jupiter token api
    // token::token_price().await; // get token price using Jupiter token api
    // token::token_info().await // get token malicious info
}
