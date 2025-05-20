# Examples for Jupiter Aggregator Rust SDK

Practical examples demonstrating how to use the [Jupiter Aggregator Rust SDK](https://crates.io/crates/jup-ag-sdk) to perform token swaps on the Solana blockchain.

## Contents

### 1. [`swap.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/swap.rs)

This example shows how to:

- Get a quote for a token swap using the **Swap API**.
- Construct and sign a transaction using your private key.
- Broadcast the transaction via a custom RPC.
- Use this if you want full control over the transaction

> &#9888; Swap API gives flexibility but requires you to handle slippage, fees, broadcasting, and error parsing manually.

### 2. [`ultra.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/ultra.rs)

This example demonstrates how to:

- Create a swap using the Ultra API, Jupiter’s newer and simplified interface.
- Decode and sign a transaction.
- Execute the transaction using ultra_execute_order.
- Use this if you prefer ease-of-use and high-level abstractions:
  - Ultra handles slippage, RPCs, optimal fees, and more.
  - Ideal for beginners or developers who want high success rates with minimal setup.

> ✅ Ultra API is recommended for most use cases unless you need deep transaction customization.

## Environment Setup

Both examples require a .env file with your wallet’s private key:

```bash
cp .env.example .env
```

```
PRIVATE_KEY=your_base58_private_key_here
```

> &#9888; Do not commit your .env or private key to version control.

## Learn More

- [Jupiter API Docs](https://dev.jup.ag/)
- [Solana Docs](https://solana.com/docs)
