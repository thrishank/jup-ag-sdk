#[cfg(test)]
use jup_ag_sdk::JupiterClient;

#[cfg(test)]
pub const BASE_URL: &str = "https://lite-api.jup.ag";
#[cfg(test)]
pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
#[cfg(test)]
pub const JUP_MINT: &str = "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN";
#[cfg(test)]
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
#[cfg(test)]
pub const TEST_AMOUNT: u64 = 1_000_000_000;
#[cfg(test)]
pub const TEST_USER_PUBKEY: &str = "EXBdeRCdiNChKyD7akt64n9HgSXEpUtpPEhmbnm4L6iH";
#[cfg(test)]
pub const DEFAULT_SLIPPAGE_BPS: u16 = 100;

#[cfg(test)]
pub fn create_test_client() -> JupiterClient {
    JupiterClient::new("https://lite-api.jup.ag")
}
