import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCounter } from "../target/types/anchor_counter";
import { expect } from "chai";

describe("anchor-counter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>;
  const counterKeypair = anchor.web3.Keypair.generate();

  it("Initializes the counter", async () => {
    const tx = await program.methods
      .initialize()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([counterKeypair])
      .rpc();

    const account = await program.account.counter.fetch(counterKeypair.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

  it("Increments the counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterKeypair.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });

    it("Decrements the counter", async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counterKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.counter.fetch(counterKeypair.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });

});