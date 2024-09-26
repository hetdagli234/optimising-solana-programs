import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { expect } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.AnchorProvider.env();
  const counter = anchor.web3.Keypair.generate();
  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {
    const tx = await program.methods
    .initialize()
    .accounts({ counter: counter.publicKey })
    .signers([counter])
    .rpc();

    console.log("Initialize transaction signature", `https://solana.fm/tx/${tx}?cluster=devnet-alpha`);

    const counterAccount = await program.account.counterData.fetch(counter.publicKey);
    expect(counterAccount.count.toNumber()).to.equal(0);
  });

  it("Incremented the count", async () => {
    const tx = await program.methods
      .increment()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();
      
    console.log("Increment transaction signature", `https://solana.fm/tx/${tx}?cluster=devnet-alpha`);

    const counterAccount = await program.account.counterData.fetch(counter.publicKey);
    expect(counterAccount.count.toNumber()).to.equal(1);
  });
});
