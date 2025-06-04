import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PollApp } from "../target/types/poll_app";
import { expect } from "chai";
import { BN } from "bn.js";

describe("poll_app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PollApp as Program<PollApp>;
  const user = provider.wallet;

  const question = "What's your favorite color?";
  const options = ["Red", "Blue", "Green", "Yellow"];
  const duration = new BN(60); // 1 min

  let userStatsPda: anchor.web3.PublicKey;
  let pollPda: anchor.web3.PublicKey;
  let pollsCount = 0;

  before(async () => {
    // Initialize user stats
    [userStatsPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_stats"), user.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser()
      .accounts({
        userStats: userStatsPda,
        user: user.publicKey,
      })
      .rpc();
  });

  it("creates a poll", async () => {
    const userStats = await program.account.userStats.fetch(userStatsPda);
    pollsCount = userStats.pollsCount;

    [pollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),
        userStatsPda.toBuffer(),
        Buffer.from(Uint8Array.of(
          pollsCount & 0xff,
          (pollsCount >> 8) & 0xff,
          (pollsCount >> 16) & 0xff,
          (pollsCount >> 24) & 0xff
        )),
      ],
      program.programId
    );

    await program.methods
      .createPoll(question, options, duration)
      .accounts({
        poll: pollPda,
        userStats: userStatsPda,
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

  it("rejects voting twice by the same user", async () => {
    try {
      await program.methods
        .vote(1)
        .accounts({
          poll: pollPda,
          user: user.publicKey,
        })
        .rpc();
      expect.fail("Should have thrown an error for double voting");
    } catch (err) {
      expect(err).to.be.instanceOf(Error);
      expect(err.message).to.include("User has already voted");
    }
  });

  it("allows a different user to vote once successfully", async () => {
    const anotherUser = anchor.web3.Keypair.generate();
  
    // Airdrop SOL to new user
    const sig = await provider.connection.requestAirdrop(
      anotherUser.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);
  
    await program.methods
      .vote(2)
      .accounts({
        poll: pollPda,
        user: anotherUser.publicKey,
      })
      .signers([anotherUser])
      .rpc();
  
    const pollAccount = await program.account.poll.fetch(pollPda);
    expect(pollAccount.votes).to.eql([0, 1, 1, 0]);
  });

  it("rejects invalid option index", async () => {
    try {
      await program.methods
        .vote(99)
        .accounts({
          poll: pollPda,
          user: user.publicKey,
        })
        .rpc();
      expect.fail("Should have thrown an error");
    } catch (err) {
      expect(err).to.be.instanceOf(Error);
      expect(err.message).to.include("Invalid option index");
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
      { option: "Green", votes: 1 },
      { option: "Yellow", votes: 0 },
    ]);
    expect(results.totalVotes).to.equal(2);
  });

  it("allows another user to create a different poll", async () => {
    const anotherUser = anchor.web3.Keypair.generate();

    // Airdrop SOL to new user
    const sig = await provider.connection.requestAirdrop(
      anotherUser.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    const [anotherUserStatsPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user_stats"), anotherUser.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeUser()
      .accounts({
        userStats: anotherUserStatsPda,
        user: anotherUser.publicKey,
      })
      .signers([anotherUser])
      .rpc();

    const otherOptions = ["Yes", "No"];
    const [anotherPollPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("poll"),
        anotherUserStatsPda.toBuffer(),
        Buffer.from(Uint8Array.of(
          0 & 0xff,
          (0 >> 8) & 0xff,
          (0 >> 16) & 0xff,
          (0 >> 24) & 0xff
        )), // First poll for this user
      ],
      program.programId
    );

    await program.methods
      .createPoll("Do you like Solana?", otherOptions, new BN(60))
      .accounts({
        poll: anotherPollPda,
        userStats: anotherUserStatsPda,
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