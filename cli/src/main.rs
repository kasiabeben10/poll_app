mod create_poll;
mod vote;
mod view_poll;
mod initialize_user;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "poll_cli")]
#[command(about = "CLI for interacting with the poll_app on Solana", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    InitializeUser,
    CreatePoll {
        question: String,
        #[arg(required = true)]
        options: Vec<String>,
        duration: i64,
    },
    Vote {
        option_index: u8,
        poll_number: Option<u32>,
    },
    ViewPoll {
        poll_number: Option<u32>,
    },
    GetWinner {
        poll_number: Option<u32>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::InitializeUser => initialize_user::handle_initialize_user()?,
        Commands::CreatePoll {
            question,
            options,
            duration,
        } => create_poll::handle_create_poll(question, options, duration)?,
        Commands::Vote { option_index, poll_number } => 
            vote::handle_vote(option_index, poll_number)?,
        Commands::ViewPoll { poll_number } => 
            view_poll::handle_view_poll(poll_number)?,
        Commands::GetWinner { poll_number } => 
            view_poll::handle_get_winner(poll_number)?,
    }

    Ok(())
}