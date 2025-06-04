mod create_poll;
mod vote;
mod view_poll;
mod initialize_user;
use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "poll")]
#[command(arg_required_else_help = true)]
#[command(version, about = "Decentralized poll app on Solana", 
          long_about = None,
          after_help = "Examples:\n  poll create-poll -q \"Favorite color\" -o Red -o Blue -o Green -d 3600\n  poll vote -o 1 -p DvUMKKX...")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    InitializeUser,
    CreatePoll {
        #[arg(short, long, required = true)]
        question: String,
        #[arg(short, long, required = true, help="List of options (at least one required)")]
        options: Vec<String>,
        #[arg(short, long, help="Poll duration in seconds (if no time requirements put 0 or skip)")]
        duration: i64,
    },
    Vote {
        #[arg(short, long, required=true, help = "Option index (starts from 1)")]
        option_index: u8,
        #[arg(short, long, required=true, help="Poll address (e.g. DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3)")]
        poll_address: String,
    },
    ViewPoll {
        #[arg(short, long, required=true, help="Poll address (e.g. DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3)")]
        poll_address: String,
    },
    GetWinner {
        #[arg(short, long, required=true, help="Poll address (e.g. DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3)")]
        poll_address: String,
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
        Commands::Vote { option_index, poll_address } => 
            vote::handle_vote(option_index, poll_address)?,
        Commands::ViewPoll { poll_address } => 
            view_poll::handle_view_poll(poll_address)?,
        Commands::GetWinner { poll_address } => 
            view_poll::handle_get_winner(poll_address)?,
    }

    Ok(())
}