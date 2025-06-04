extern crate solana_sdk;

use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;
use dirs::home_dir;

pub fn handle_view_poll(poll_number: Option<u32>) -> anyhow::Result<()> {
    let keypair_path = home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

    let payer = read_keypair_file(keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "8hLpnr7jBwD3UsS5DvbQF4mLK6qzyg6KQFmePsJrwMR5".parse()?;
    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    // Get user stats to get poll count
    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );
    let user_stats: poll_app::UserStats = program.account(user_stats_pda)?;

    let poll_number = poll_number.unwrap_or(user_stats.poll_count);
    if poll_number == 0 || poll_number > user_stats.poll_count {
        return Err(anyhow::anyhow!("Invalid poll number"));
    }

    let (poll_pda, _) = Pubkey::find_program_address(
        &[b"poll", &program.payer().to_bytes(), &(poll_number - 1).to_le_bytes()],
        &program_id,
    );
    
    let poll_account: poll_app::Poll = program.account(poll_pda)?;

    println!("Poll #{}", poll_number);
    println!("Question: {}", poll_account.question);
    for (i, option) in poll_account.options.iter().enumerate() {
        println!("{}: {} - {} votes", i, option, poll_account.votes[i]);
    }
    println!("Total voters: {}", poll_account.voters.len());

    Ok(())
}

pub fn handle_get_winner(poll_number: Option<u32>) -> anyhow::Result<()> {
    let keypair_path = home_dir()
        .expect("Could not find home directory")
        .join(".config/solana/id.json");

    let payer = read_keypair_file(keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "8hLpnr7jBwD3UsS5DvbQF4mLK6qzyg6KQFmePsJrwMR5".parse()?;
    let client = Client::new_with_options(Cluster::Localnet, Rc::new(payer), CommitmentConfig::processed());
    let program = client.program(program_id)?;
    
    // Get user stats to get poll count
    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );
    let user_stats: poll_app::UserStats = program.account(user_stats_pda)?;
    
    let poll_number = poll_number.unwrap_or(user_stats.poll_count);
    if poll_number == 0 || poll_number > user_stats.poll_count {
        return Err(anyhow::anyhow!("Invalid poll number"));
    }
    
    let (poll_pda, _) = Pubkey::find_program_address(
        &[b"poll", &program.payer().to_bytes(), &(poll_number - 1).to_le_bytes()],
        &program_id,
    );
    let poll_account: poll_app::Poll = program.account(poll_pda)?;
    
    let max_votes = *poll_account.votes.iter().max().unwrap_or(&0);
    let winners: Vec<String> = poll_account
        .options
        .iter()
        .zip(poll_account.votes.iter())
        .filter(|&(_, &votes)| votes == max_votes)
        .map(|(option, _)| option.clone())
        .collect();
    
    println!("Winning options ({} votes):", max_votes);
    for option in winners {
        println!("- {}", option);
    }
    
    Ok(())
}