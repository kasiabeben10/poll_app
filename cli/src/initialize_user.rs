use anchor_client::solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use anchor_client::{Client, Cluster};
use dirs::home_dir;
use solana_sdk::commitment_config::CommitmentConfig;
use std::rc::Rc;

pub fn handle_initialize_user() -> anyhow::Result<()> {
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

    if program.account::<poll_app::UserStats>(user_stats_pda).is_ok() {
        println!("User already initialized at: {}", user_stats_pda);
        return Ok(());
    }

    let request = program
        .request()
        .accounts(poll_app::accounts::InitializeUser {
            user_stats: user_stats_pda,
            user: program.payer(),
            system_program: anchor_lang::system_program::ID,
        })
        .args(poll_app::instruction::InitializeUser {});

    match request.send() {
        Ok(_) => {
            println!("User stats initialized at: {}", user_stats_pda);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to initialize user: {}", e);
            Err(anyhow::anyhow!("Initialization failed"))
        }
    }
}