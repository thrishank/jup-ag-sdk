use jup_ag_sdk::JupiterClient;

pub const BASE_URL: &str = "https://lite-api.jup.ag";
pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
pub const JUP_MINT: &str = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
pub const TEST_AMOUNT: u64 = 1_000_000_000;
pub const TEST_USER_PUBKEY: &str = "EXBdeRCdiNChKyD7akt64n9HgSXEpUtpPEhmbnm4L6iH";
pub const DEFAULT_SLIPPAGE_BPS: u16 = 100;

pub fn create_test_client() -> JupiterClient {
    JupiterClient::new(BASE_URL)
}
