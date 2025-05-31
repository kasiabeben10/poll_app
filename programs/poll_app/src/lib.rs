use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;

declare_id!("9DDurAcNao6xJQNZwDbuB7E4RVTdPsoaiY93358JAkdG");

#[program]
pub mod poll_app {
    use super::*;

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        question: String,
        options: Vec<String>,
        duration: i64, // in seconds
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.question = question;
        poll.options = options;
        poll.votes = vec![0; poll.options.len()];
        poll.bump = ctx.bumps.poll;
        poll.created_at = Clock::get()?.unix_timestamp;
        poll.duration = duration;
        poll.voters = vec![];
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option_index: u8) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        let user = ctx.accounts.user.key();
    
        let now = Clock::get()?.unix_timestamp;
        if poll.duration > 0 && now > poll.created_at + poll.duration {
            return Err(ErrorCode::PollClosed.into());
        }
    
        require!(
            (option_index as usize) < poll.options.len(),
            ErrorCode::InvalidOption
        );

        if poll.voters.contains(&user) {
            return Err(ErrorCode::AlreadyVoted.into());
        }
    
        poll.votes[option_index as usize] += 1;
        poll.voters.push(user);
        Ok(())
    }    
    

    pub fn get_results(ctx: Context<GetResults>) -> Result<PollResults> {
        let poll = &ctx.accounts.poll;
    
        let total_votes = poll.votes.iter().sum::<u32>();
        let paired_results: Vec<PollResultItem> = poll
            .options
            .iter()
            .zip(poll.votes.iter())
            .map(|(option, &votes)| PollResultItem {
                option: option.clone(),
                votes,
            })
            .collect();
    
        Ok(PollResults {
            question: poll.question.clone(),
            results: paired_results,
            total_votes,
        })
    }

    pub fn get_winner(ctx: Context<GetResults>) -> Result<Winner> {
        let poll = &ctx.accounts.poll;
        let max_votes = *poll.votes.iter().max().unwrap_or(&0);
    
        let winners: Vec<String> = poll
            .options
            .iter()
            .zip(poll.votes.iter())
            .filter(|&(_, &votes)| votes == max_votes)
            .map(|(option, _)| option.clone())
            .collect();
    
        Ok(Winner {
            options: winners,
            votes: max_votes,
        })
    }
    
}
    

#[account]
pub struct Poll {
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub voters: Vec<Pubkey>,
    pub bump: u8,
    pub created_at: i64,
    pub duration: i64,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 256 + 4 + (4 + 256) * 4 + 4 + 4 * 4 + 4 + (32 * 100) + 1 + 8 + 8,
        seeds = [b"poll", user.key().as_ref()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll: Account<'info, Poll>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetResults<'info> {
    #[account()]
    pub poll: Account<'info, Poll>,
    pub user: Signer<'info>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct PollResults {
    pub question: String,
    pub results: Vec<PollResultItem>,
    pub total_votes: u32,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct PollResultItem {
    pub option: String,
    pub votes: u32,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Winner {
    pub options: Vec<String>,
    pub votes: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid option index")]
    InvalidOption,
    #[msg("Poll is closed")]
    PollClosed,
    #[msg("User has already voted")]
    AlreadyVoted,
}