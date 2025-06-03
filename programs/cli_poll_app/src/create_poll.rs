use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use dirs::home_dir;
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_create_poll(
    question: String,
    options: Vec<String>,
    duration: i64,
) -> anyhow::Result<()> {
    let keypair_path = home_dir()
    .expect("Could not find home directory")
    .join(".config/solana/id.json");

    let payer = read_keypair_file(keypair_path)
    .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "9DDurAcNao6xJQNZwDbuB7E4RVTdPsoaiY93358JAkdG".parse()?;
    let client = Client::new_with_options(Cluster::Devnet, Rc::new(payer), CommitmentConfig::processed());
    let program = client.program(program_id)?;

    let poll_pda = Pubkey::find_program_address(&[b"poll", &program.payer().to_bytes()], &program_id).0;

    program
        .request()
        .accounts(anchor_lang::ToAccountMetas::to_account_metas(
            &poll_app::accounts::CreatePoll {
                poll: poll_pda,
                user: program.payer(),
                system_program: program_id
            },
            None,
        ))
        .args(poll_app::instruction::CreatePoll {
            question,
            options,
            duration,
        })
        .send()?;

    println!("Poll created at: {}", poll_pda);
    Ok(())
}
