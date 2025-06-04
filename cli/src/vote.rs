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
    let program_id: Pubkey = "8hLpnr7jBwD3UsS5DvbQF4mLK6qzyg6KQFmePsJrwMR5".parse()?;
    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );
    
    let user_stats = match program.account::<poll_app::UserStats>(user_stats_pda) {
        Ok(stats) => stats,
        Err(_) => return Err(anyhow::anyhow!("User not initialized")),
    };

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
    println!("✅ You voted for option {} in poll {}", option_index + 1, poll_pda);
    Ok(())
}