use bincode::deserialize;
use jup_ag_sdk::{
    JupiterClient,
    types::{QuoteGetSwapModeEnum, QuoteRequest, SwapRequest, SwapResponse},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, Signer},
    transaction::VersionedTransaction,
};

#[tokio::main]
async fn main() {
    let client = JupiterClient::new("https://lite-api.jup.ag/swap/v1");

    let quote = QuoteRequest::new(
        "So11111111111111111111111111111111111111112",
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        1_000_000, // 1 USDC (6 decimals)
    )
    .swap_mode(QuoteGetSwapModeEnum::ExactOut); // Swap some SOL fro exact 1 USDC

    let quote_res = client.get_quote(quote).await.expect("Failed to get quote");

    let payload = SwapRequest::new("input_your_wallet_address", quote_res);
    let swap_res: SwapResponse = client
        .get_swap_transaction(payload)
        .await
        .expect("Failed to get swap transaction");

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
