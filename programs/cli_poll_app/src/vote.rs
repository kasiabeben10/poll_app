use anchor_client::solana_sdk::{pubkey::Pubkey};
use solana_sdk::signature::read_keypair_file;
use anchor_client::{Client, Cluster};
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_vote(option_index: u8) -> anyhow::Result<()> {
    let payer = read_keypair_file("~/.config/solana/id.json")
    .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "9DDurAcNao6xJQNZwDbuB7E4RVTdPsoaiY93358JAkdG".parse()?;
    let client = Client::new_with_options(Cluster::Devnet, Rc::new(payer), CommitmentConfig::processed());
    let program = client.program(program_id)?;

    let poll_pda = Pubkey::find_program_address(&[b"poll", &program.payer().to_bytes()], &program_id).0;

    program
        .request()
        .accounts(anchor_lang::ToAccountMetas::to_account_metas(
            &poll_app::accounts::Vote {
                poll: poll_pda,
                user: program.payer(),
            },
            None,
        ))
        .args(poll_app::instruction::Vote { option_index })
        .send()?;

    println!("Vote submitted.");
    Ok(())
}
