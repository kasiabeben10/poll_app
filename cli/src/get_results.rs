use anchor_client::{Client, Cluster};
use anchor_client::solana_sdk::signature::read_keypair_file;
use std::rc::Rc;
use dirs::home_dir;

pub async fn handle() {
    let keypair_path = home_dir()
    .expect("Could not find home directory")
    .join(".config/solana/id.json");

    let payer = read_keypair_file(keypair_path)
    .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let client = Client::new(Cluster::Devnet, Rc::new(payer));
    let program = client.program(poll_app::ID);

    let (poll_pda, _) = Pubkey::find_program_address(&[b"poll", &program.payer().to_bytes()], &poll_app::ID);
    let poll: poll_app::Poll = program.account(poll_pda).unwrap();

    for (i, (opt, &votes)) in poll.options.iter().zip(poll.votes.iter()).enumerate() {
        println!("{}. {} - {} votes", i + 1, opt, votes);
    }
}