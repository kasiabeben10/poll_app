use anchor_lang::prelude::*;

declare_id!("9DDurAcNao6xJQNZwDbuB7E4RVTdPsoaiY93358JAkdG");

#[program]
pub mod poll_app {
    use super::*;

    pub fn create_poll(
        ctx: Context<CreatePoll>,
        question: String,
        options: Vec<String>,
    ) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        poll.question = question;
        poll.options = options;
        poll.votes = vec![0; poll.options.len()];
        poll.bump = ctx.bumps.poll; // Correct way to access the bump
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, option_index: u8) -> Result<()> {
        let poll = &mut ctx.accounts.poll;
        
        require!(
            (option_index as usize) < poll.options.len(),
            ErrorCode::InvalidOption
        );
        
        poll.votes[option_index as usize] += 1;
        Ok(())
    }
}

#[account]
pub struct Poll {
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<u32>,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + 256 + 4 + (4 + 256) * 4 + 4 + 4 * 4 + 1,
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

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid option index")]
    InvalidOption,
}