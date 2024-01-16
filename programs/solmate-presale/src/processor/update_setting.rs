use crate::{constant::*, state::*};
use anchor_lang::prelude::*;

pub fn handle(
    ctx: Context<UpdateSetting>,
    start_date: Option<u64>,
    end_date: Option<u64>,
    activated: Option<bool>,
) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;

    global_state.authority = ctx.accounts.new_authority.key();

    if let Some(_start_date) = start_date {
        global_state.start_date = _start_date;
    }
    if let Some(_end_date) = end_date {
        global_state.end_date = _end_date;
    }
    if let Some(_activated) = activated {
        global_state.activated = _activated;
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct UpdateSetting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECKED: new authority account
    pub new_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [PREFIX],
        bump,
        has_one = authority
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub system_program: Program<'info, System>,
}
