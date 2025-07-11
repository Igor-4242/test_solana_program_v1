import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SwapTokenProgram } from "../target/types/swap_token_program";
import { assert } from "chai";

describe("swap_token_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.swapTokenProgram as Program<SwapTokenProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is initialized 2!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature 2", tx);
  });

  it("Initializes the program", async () => {
    // Call the initialize instruction
    const tx = await program.methods
      .initialize()
      .accounts({
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Verify transaction signature exists
    assert.ok(tx, "Transaction signature should exist");

    // Fetch transaction logs to verify the program message
    const client = provider.connection;
    const txDetails = await client.getTransaction(tx, {
      commitment: "confirmed",
      maxSupportedTransactionVersion: 0,
    });

    // Check for the expected log message
    const logMessage = `Program log: Greetings from: ${program.programId.toString()}`;
    assert.ok(
      txDetails.meta.logMessages.some((log) => log.includes("Greetings from")),
      `Log should contain "${logMessage}"`
    );

    console.log("Transaction signature:", tx);
  });

});
