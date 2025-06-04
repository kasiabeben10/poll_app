use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use dirs::home_dir;
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_view_poll(poll_address: String) -> anyhow::Result<()> {
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

    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );
    
    let user_stats = match program.account::<poll_app::UserStats>(user_stats_pda) {
        Ok(stats) => stats,
        Err(_) => return Err(anyhow::anyhow!("User not initialized")),
    };

    let poll_pda: Pubkey = poll_address.parse()?;
    let poll_account: poll_app::Poll = program.account(poll_pda)?;

    println!("📊 Poll Results 📊");
    println!("🔹 Question: {}", poll_account.question);
    println!("\nOptions:");
    for (i, option) in poll_account.options.iter().enumerate() {
        println!("{}: {} - {} votes", i, option, poll_account.votes[i]);
    }
    println!("-----------\n");
    println!("Total voters: {}", poll_account.voters.len());

    Ok(())
}

pub fn handle_get_winner(poll_address: String) -> anyhow::Result<()> {
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

    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );
    
    let user_stats = match program.account::<poll_app::UserStats>(user_stats_pda) {
        Ok(stats) => stats,
        Err(_) => return Err(anyhow::anyhow!("User not initialized")),
    };

    let poll_pda: Pubkey = poll_address.parse()?;
    let poll_account: poll_app::Poll = program.account(poll_pda)?;
    
    let max_votes = *poll_account.votes.iter().max().unwrap_or(&0);
    let winners: Vec<String> = poll_account
        .options
        .iter()
        .zip(poll_account.votes.iter())
        .filter(|&(_, &votes)| votes == max_votes)
        .map(|(option, _)| option.clone())
        .collect();
    
    println!("🏆 Winner(s) 🏆");
    println!("Total votes: {}", poll_account.voters.len());
    println!("Winning options ({} votes each):", max_votes);
    for option in winners {
        println!("  ✨ {}", option);
    }
    
    Ok(())
}