use crate::{constant::*, state::*};
use anchor_lang::prelude::*;

pub fn handle(ctx: Context<Initialize>, start_date: u64, end_date: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;

    global_state.authority = ctx.accounts.authority.key();
    global_state.start_date = start_date;
    global_state.end_date = end_date;
    global_state.activated = true;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [PREFIX],
        bump,
        payer = authority,
        space = 8 + 1000,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub system_program: Program<'info, System>,
}
