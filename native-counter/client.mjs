import {
    Connection,
    Keypair,
    PublicKey,
    Transaction,
    TransactionInstruction,
    SystemProgram,
    sendAndConfirmTransaction,
} from "@solana/web3.js";
import feePayerWallet from './feePayer-wallet.json' with { type: "json" };
const programId = new PublicKey("Guj9e8P2Y6WFPAWMJdDcK7bSRhuMMyoJTFpKX4ecJUo1");
// Connect to the Solana devnet
const connection = new Connection("https://api.devnet.solana.com", "confirmed");
// Create a new keypair for the counter account
const counterKeypair = Keypair.generate();
async function initialize() {
    //Imported a private key from a file for paying fees,you can use the default path for your solana config: ~/.config/solana/id.json
    const payer = Keypair.fromSecretKey(new Uint8Array(feePayerWallet));
    const tx = new Transaction().add(
        new TransactionInstruction({
            programId: programId,
            keys: [
                { pubkey: counterKeypair.publicKey, isSigner: true, isWritable: true },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            data: Buffer.from([0]), // 0 for initialize instruction
        })
    );
    const txHash = await sendAndConfirmTransaction(connection, tx, [payer, counterKeypair]);
    console.log("Initialize transaction signature", `https://solana.fm/tx/${txHash}?cluster=devnet-alpha`);
    // Fetch the account data to verify initialization
    const accountInfo = await connection.getAccountInfo(counterKeypair.publicKey);
    const count = accountInfo.data.readBigUInt64LE(0);
    console.log("Initial count:", count);
    if (count !== 0n) {
        throw new Error("Initialization failed: count is not 0");
    }
    console.log("Initialization successful!");
}
async function increment() {
    const payer = Keypair.fromSecretKey(new Uint8Array(feePayerWallet));
    const tx = new Transaction().add(
        new TransactionInstruction({
            programId: programId,
            keys: [
                { pubkey: counterKeypair.publicKey, isSigner: false, isWritable: true },
                { pubkey: payer.publicKey, isSigner: true, isWritable: false },
            ],
            data: Buffer.from([1]), // 1 for increment instruction
        })
    );
    const txHash = await sendAndConfirmTransaction(connection, tx, [payer]);
    console.log("Increment transaction signature", `https://solana.fm/tx/${txHash}?cluster=devnet-alpha`);
    // Fetch the account data to verify increment
    const accountInfo = await connection.getAccountInfo(counterKeypair.publicKey);
    const count = accountInfo.data.readBigUInt64LE(0);
    console.log("New count:", count);
    if (count !== 1n) {
        throw new Error("Increment failed: count is not 1");
    }
    console.log("Increment successful!");
}
async function runTests() {
    try {
        await initialize();
        await increment();
        console.log("All tests passed successfully!");
    } catch (error) {
        console.error("Test failed:", error);
    }
}
runTests();