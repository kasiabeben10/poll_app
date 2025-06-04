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

    let payer = read_keypair_file(&keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair: {}", e))?;
    let program_id: Pubkey = "8hLpnr7jBwD3UsS5DvbQF4mLK6qzyg6KQFmePsJrwMR5".parse()?;
    let client = Client::new_with_options(
        Cluster::Localnet,
        Rc::new(payer),
        CommitmentConfig::processed(),
    );
    let program = client.program(program_id)?;

    // Get or initialize user stats
    let (user_stats_pda, _) = Pubkey::find_program_address(
        &[b"user_stats", &program.payer().to_bytes()],
        &program_id,
    );

    let user_stats = match program.account::<poll_app::UserStats>(user_stats_pda) {
        Ok(stats) => stats,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "User not initialized. Please run `initialize-user` first."
            ));
        }
    };

    let polls_count = user_stats.polls_count;
    
    let (poll_pda, _) = Pubkey::find_program_address(
        &[b"poll", &user_stats_pda.to_bytes(), &polls_count.to_le_bytes()],
        &program_id,
    );

    let request = program
        .request()
        .accounts(poll_app::accounts::CreatePoll {
            poll: poll_pda,
            user: program.payer(),
            user_stats: user_stats_pda,
            system_program: anchor_lang::system_program::ID,
        })
        .args(poll_app::instruction::CreatePoll {
            question: question.clone(),
            options: options.clone(),
            duration: duration.clone(),
        });

    match request.send() {
        Ok(_) => {
            println!("✅ Poll created successfully!");
            println!("🔗 Poll address: {}", poll_pda);
            println!("📝 Question: {}", question);
            println!("📌 Options:");
            for (i, option) in options.iter().enumerate() {
                println!("  {}. {}", i + 1, option);
            }
            println!("Validity: {} min", duration/60 );

            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to create poll: {}", e);
            Err(anyhow::anyhow!("Poll creation failed"))
        }
    }
}