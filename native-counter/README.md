# Native Counter Program (Rust)

This is an implementation of a simple counter program using native Rust without the Anchor framework.

## Setup

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install
2. Install Solana CLI: https://docs.solana.com/cli/install-solana-cli-tools
3. Set up a Solana wallet:
   ```
   solana-keygen new -o feePayer-wallet.json
   solana airdrop 2 $(solana-keygen pubkey feePayer-wallet.json) --url devnet
   ```

## Building and Deploying the Program

1. Build the program:
   ```
   cargo build-sbf
   ```

2. Deploy the program:
   ```
   solana program deploy target/deploy/native_counter.so
   ```

## Testing the Program

1. Install dependencies for testing:
   ```
   npm install @solana/web3.js
   ```

2. Run the test client:
   ```
   node client.mjs
   ```