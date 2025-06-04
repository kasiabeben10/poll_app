use anchor_lang::prelude::*;
use anchor_lang::AnchorSerialize;
use anchor_lang::solana_program::hash::hash as sha256;

declare_id!("8hLpnr7jBwD3UsS5DvbQF4mLK6qzyg6KQFmePsJrwMR5");

#[program]
pub mod poll_app {
    use super::*;

    pub fn initialize_user_stats(ctx: Context<InitializeUserStats>) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.poll_count = 0;
        Ok(())
    }

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        question: String,
        options: Vec<String>,
        duration: i64, // in seconds
    ) -> Result<()> {
        require!(question.len() <= 256, ErrorCode::QuestionTooLong);
        require!(!question.is_empty(), ErrorCode::EmptyQuestion);
        require!(options.len() >= 1, ErrorCode::NotEnoughOptions);
        require!(options.len() <=5, ErrorCode::TooMuchOptions);
        require!(options.iter().all(|o| !o.is_empty()), ErrorCode::EmptyOption);
        require!(duration >= 0, ErrorCode::InvalidDuration);
        
        
        let user_stats = &mut ctx.accounts.user_stats;
        user_stats.poll_count += 1;

        let poll = &mut ctx.accounts.poll;
        poll.question = question;
        poll.options = options;
        poll.votes = vec![0; poll.options.len()];
        poll.bump = ctx.bumps.poll;
        poll.created_at = Clock::get()?.unix_timestamp;
        poll.duration = duration;
        poll.voters = Vec::new();
        poll.voter_count = 0;
        poll.seed = Clock::get()?.unix_timestamp.to_le_bytes();
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

        let user_seed = sha256(&user.to_bytes()).to_bytes();
        let poll_seed = sha256(&poll.seed).to_bytes();
        
        // Create a combined array by concatenating the two 32-byte arrays
        let mut combined = Vec::with_capacity(64);
        combined.extend_from_slice(&user_seed);
        combined.extend_from_slice(&poll_seed);
        
        let commitment = sha256(&combined).to_bytes();
        
        if poll.voters.contains(&commitment) {
            return Err(ErrorCode::AlreadyVoted.into());
        }
    
        poll.votes[option_index as usize] += 1;
        poll.voter_count += 1;
        poll.voters.push(commitment);
        
        emit!(VoteEvent {
            user,
            poll: ctx.accounts.poll.key(),
            option_index,
            timestamp: Clock::get()?.unix_timestamp,
        });
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
}

#[event]
pub struct VoteEvent {
    pub user: Pubkey,
    pub poll: Pubkey,
    pub option_index: u8,
    pub timestamp: i64,
}

#[account]
pub struct Poll {
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub voters: Vec<[u8; 32]>,
    pub bump: u8,
    pub created_at: i64,
    pub duration: i64,
    pub voter_count: u32,
    pub seed: [u8; 8], // Using 8 bytes for seed (timestamp)
}

impl Poll {
    pub fn calculate_space(options: &[String], max_voters: usize) -> usize {
        // Discriminator
        let mut space = 8;
        
        // Question: 4 (length) + 256 (max text)
        space += 4 + 256;
        
        // Options: 4 (length) + sum (4 + string length) for each option
        space += 4 + options.iter().map(|o| 4 + o.len()).sum::<usize>();
        
        // Votes: 4 (length) + 4 * number of options
        space += 4 + options.len() * 4;
        
        // voters: 4 (length) + 32 * max_voters
        space += 4 + max_voters * 32;
        
        // Other fields: bump (1) + created_at (8) + duration (8) + voter_count (4) + seed (8)
        space += 1 + 8 + 8 + 4 + 8;
        
        space
    }
}

#[derive(Accounts)]
pub struct InitializeUserStats<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4, // 8 for discriminator, 4 for u32
        seeds = [b"user_stats", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserStats>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserStats {
    pub poll_count: u32,
}

#[derive(Accounts)]
#[instruction(question: String, options: Vec<String>)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = user,
        space = 8 +                  // Anchor discriminator
               4 + question.len() +  // Question (4-byte length + string)
               4 +                   // Options vector length
               options.iter().map(|o| 4 + o.len()).sum::<usize>() + // Each option (4-byte length + string)
               4 +                   // Votes vector length
               options.len() * 4 +   // Each vote (u32)
               4 +                   // Voters vector length
               32 * 10 +             // Max 10 voters (Pubkey is 32 bytes)
               8,                    // end_time (i64)
        seeds = [b"poll", user.key().as_ref(), &user_stats.poll_count.to_le_bytes()],
        bump
    )]
    pub poll: Account<'info, Poll>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, seeds=[b"user_stats", user.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserStats>,
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

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid option index")]
    InvalidOption,
    #[msg("Poll is closed")]
    PollClosed,
    #[msg("User has already voted")]
    AlreadyVoted,
    #[msg("Too long question")]
    QuestionTooLong,
    #[msg("Empty question")]
    EmptyQuestion,
    #[msg("No options")]
    NotEnoughOptions,
    #[msg("Empty option")]
    EmptyOption,
    #[msg("Negative duration")]
    InvalidDuration,
    #[msg("More than 5 options")]
    TooMuchOptions
}