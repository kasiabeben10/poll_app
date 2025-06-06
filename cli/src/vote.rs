use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use dirs::home_dir;
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_vote(option_index: u8, poll_address: String) -> anyhow::Result<()> {
    let keypair_path = home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

    let payer = read_keypair_file(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "6rN7v7FDj9ub6Qvj3cpw7CxhziDy6izMMYWnTwSFfMFY".parse()?;
    let client = Client::new_with_options(
        Cluster::Devnet,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    let poll_pda: Pubkey = poll_address.parse()?;

    program
        .request()
        .accounts(poll_app::accounts::Vote {
            poll: poll_pda,
            user: program.payer(),
        })
        .args(poll_app::instruction::Vote { option_index })
        .send()?;

    println!("🗳️ Your vote has been submitted successfully!");
    println!("✅ You voted for option {} in poll {}", option_index, poll_pda);
    Ok(())
}