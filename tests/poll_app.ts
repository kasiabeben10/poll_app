import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PollApp } from "../target/types/poll_app";
import { expect } from "chai";

describe("poll_app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PollApp as Program<PollApp>;
  const user = provider.wallet;

  const question = "What's your favorite color?";
  const options = ["Red", "Blue", "Green", "Yellow"];

  let pollPda: anchor.web3.PublicKey;

  before(async () => {
    [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("creates a poll", async () => {
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

  it("returns correct poll results", async () => {
    const results = await program.methods
      .getResults()
      .accounts({
        poll: pollPda,
        user: user.publicKey,
      })
      .view();

    expect(results.question).to.equal(question);
    expect(results.results).to.eql([
      { option: "Red", votes: 0 },
      { option: "Blue", votes: 1 },
      { option: "Green", votes: 0 },
      { option: "Yellow", votes: 0 },
    ]);
    expect(results.totalVotes).to.equal(1);
  });

  it("allows another user to create a different poll", async () => {
    const anotherUser = anchor.web3.Keypair.generate();

    // airdrop sol to new user
    const sig = await provider.connection.requestAirdrop(
      anotherUser.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    const [anotherPollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("poll"), anotherUser.publicKey.toBuffer()],
      program.programId
    );

    const otherOptions = ["Yes", "No"];

    await program.methods
      .createPoll("Do you like Solana?", otherOptions)
      .accounts({
        poll: anotherPollPda,
        user: anotherUser.publicKey,
      })
      .signers([anotherUser])
      .rpc();

    const pollAccount = await program.account.poll.fetch(anotherPollPda);
    expect(pollAccount.question).to.equal("Do you like Solana?");
    expect(pollAccount.options).to.eql(otherOptions);
    expect(pollAccount.votes).to.eql([0, 0]);
  });
});
