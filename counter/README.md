# Counter Program (Anchor)

This is an implementation of a simple counter program using the Anchor framework.

## Setup

1. Install Rust and Cargo: https://www.rust-lang.org/tools/install
2. Install Solana CLI: https://docs.solana.com/cli/install-solana-cli-tools
3. Install Anchor: https://www.anchor-lang.com/docs/installation
4. Set up a Solana wallet:
   ```
   solana-keygen new -o id.json
   ```

## Building and Deploying the Program

1. Install project dependencies:
   ```
   npm install
   ```

2. Build the program:
   ```
   anchor build
   ```

3. Get the program ID:
   ```
   solana address -k target/deploy/counter-keypair.json
   ```

4. Update the program ID in `Anchor.toml` and `lib.rs` with the address from step 3.

5. Deploy the program to devnet:
   ```
   solana config set --url devnet
   anchor deploy
   ```

## Testing the Program

1. Ensure you have some SOL in your devnet wallet:
   ```
   solana airdrop 2 <YOUR_WALLET_ADDRESS> --url devnet
   ```

2. Run the tests:
   ```
   anchor test --provider.cluster devnet
   ```