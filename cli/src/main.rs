mod create_poll;
mod vote;
mod view_poll;
extern crate clap;
extern crate solana_sdk;
extern crate anchor_client;
extern crate dirs;

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
    CreatePoll {
        question: String,
        #[arg(required = true)]
        options: Vec<String>,
        duration: i64,
    },
    Vote {
        option_index: u8,
    },
    ViewPoll,
    GetWinner,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::CreatePoll {
            question,
            options,
            duration,
        } => create_poll::handle_create_poll(question, options, duration)?,
        Commands::Vote { option_index } => vote::handle_vote(option_index)?,
        Commands::ViewPoll => view_poll::handle_view_poll()?,
        Commands::GetWinner => view_poll::handle_get_winner()?,
    }

    Ok(())
}