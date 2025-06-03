extern crate solana_sdk;



use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_view_poll() -> anyhow::Result<()> {
    let payer = read_keypair_file("~/.config/solana/id.json")
    .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "9DDurAcNao6xJQNZwDbuB7E4RVTdPsoaiY93358JAkdG".parse()?;
    let client = Client::new_with_options(Cluster::Devnet, Rc::new(payer), CommitmentConfig::processed());
    let program = client.program(program_id)?;

    let poll_pda = Pubkey::find_program_address(&[b"poll", &program.payer().to_bytes()], &program_id).0;
    let poll_account: poll_app::Poll = program.account(poll_pda)?;

    println!("Question: {}", poll_account.question);
    for (i, option) in poll_account.options.iter().enumerate() {
        println!("{}: {} - {} votes", i, option, poll_account.votes[i]);
    }
    println!("Total voters: {}", poll_account.voters.len());

    Ok(())
}

pub fn handle_get_winner() -> anyhow::Result<()> {
    let payer = read_keypair_file("~/.config/solana/id.json")
    .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "TwójProgramIDTu".parse()?;
    let client = Client::new_with_options(Cluster::Devnet, Rc::new(payer), CommitmentConfig::processed());
    let program = client.program(program_id)?;

    let poll_pda = Pubkey::find_program_address(&[b"poll", &program.payer().to_bytes()], &program_id).0;
    let simulation_result = program
        .request()
        .args(poll_app::instruction::GetWinner {})
        .send()?;

    println!("Winner raw response: {:#?}", simulation_result);
    // Możesz rozkodować to ładniej jeśli wiesz jak wygląda struktura wyniku

    Ok(())
}
