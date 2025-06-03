# Examples for Jupiter Aggregator Rust SDK

Practical examples demonstrating how to use the [Jupiter Aggregator Rust SDK](https://crates.io/crates/jup-ag-sdk).

## Environment Setup

require a .env file with your wallet’s private key to sign the transactions

```bash
cp .env.example .env
```

```
PRIVATE_KEY=your_base58_private_key_here
```

> &#9888; Do not commit your .env or private key to git.

## Contents

### 1. [`swap.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/swap.rs)

This example shows how to:

- Get a quote for a token swap using the **Swap API**.
- Construct and sign a transaction using your private key.
- Broadcast the transaction via a custom RPC.
- Use this if you want full control over the transaction

> &#9888; Swap API gives flexibility but requires you to handle slippage, fees, transaction broadcasting, and error parsing manually.

### 2. [`ultra.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/ultra.rs)

This example demonstrates how to:

- Create a swap using the Ultra API, Jupiter’s newer and simplified interface.
- Decode and sign a transaction.
- Execute the transaction using ultra_execute_order.
- Use this if you prefer ease-of-use and high-level abstractions:
  - Ultra handles slippage, RPCs, optimal fees, and more.
  - Ideal for beginners or developers who want high success rates with minimal setup.

> ✅ Ultra API is recommended for most use cases unless you need deep transaction customization.

### 3. [`token.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/token.rs)

This example demonstrates how to:

- Fetch token balances for a given wallet address.
- Query current token prices in USDC and in other tokens (e.g., SOL).
- Check token warnings using Jupiter’s shield API
- check token_api.rs docs for more token-related functions.

### 4. [`trigger.rs`](https://github.com/thrishank/jup-ag-sdk/blob/main/examples/src/trigger.rs)

This example demonstrates how to:

- create a trigger order using the Trigger API.
- sign the transaction and execute the trigger order
- cancel a trigger order
- get a user trigger order history

## Learn More

- [Jupiter API Docs](https://dev.jup.ag/)
- [Solana Docs](https://solana.com/docs)
