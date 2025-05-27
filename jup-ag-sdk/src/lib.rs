//! # Jupiter Client
//!
//! A Rust client library for interacting with Jupiter Aggregator APIs.
//!
//! This library provides a convenient interface to:
//! - Get swap quotes and execute swaps
//! - Access Ultra API features (orders, balances, shield)
//! - Fetch token prices and router information
//!
//! ## Example
//!
//! ```rust
//! use jup_ag_sdk::JupiterClient;
//! use jup_ag_sdk::types::QuoteRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = JupiterClient::new("https://lite-api.jup.ag");
//!     
//!     let quote_request = QuoteRequest::new(
//!         "So11111111111111111111111111111111111111112", // SOL
//!         "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN", // JUP
//!         1_000_000_000 // 1 SOL
//!     );
//!     
//!     let quote = client.get_quote(&quote_request).await?;
//!     println!("Quote: {:?}", quote);
//!     
//!     Ok(())
//! }
//! ```

pub use client::JupiterClient;
pub use error::JupiterClientError;

pub mod client;
pub mod error;
pub mod types;
