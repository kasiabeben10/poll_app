import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PollApp } from "../target/types/poll_app";
import { expect } from "chai";

describe("poll_app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PollApp as Program<PollApp>;
  const user = provider.wallet;

  it("creates a poll", async () => {
    const question = "What's your favorite color?";
    const options = ["Red", "Blue", "Green", "Yellow"];

    const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .createPoll(question, options)
      .accounts({
        poll: pollPda,
        user: user.publicKey,
      })
      .rpc();

    const pollAccount = await program.account.poll.fetch(pollPda);
    expect(pollAccount.question).to.equal(question);
    expect(pollAccount.options).to.eql(options);
    expect(pollAccount.votes).to.eql([0, 0, 0, 0]);
  });

  it("allows voting on a poll", async () => {
    const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), user.publicKey.toBuffer()],
      program.programId
    );

    // Vote for option 1 (Blue)
    await program.methods
      .vote(1)
      .accounts({
        poll: pollPda,
        user: user.publicKey,
      })
      .rpc();

    const pollAccount = await program.account.poll.fetch(pollPda);
    expect(pollAccount.votes).to.eql([0, 1, 0, 0]);
  });

  it("rejects invalid option index", async () => {
    const [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), user.publicKey.toBuffer()],
      program.programId
    );

    try {
      await program.methods
        .vote(99) // Invalid index
        .accounts({
          poll: pollPda,
          user: user.publicKey,
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err.error.errorMessage).to.include("Invalid option index");
    }
  });
});