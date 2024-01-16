use crate::{constant::*, error::*, state::*, utils::*};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction},
};
use anchor_spl::token::{Mint, Token};

pub fn handle(ctx: Context<Buy>, token_amount: u64, amount: u64) -> Result<()> {
    let sale_state = &mut ctx.accounts.sale_state;

    let is_zero = is_zero_account(&sale_state.to_account_info());
    if is_zero {
        sale_state.authority = ctx.accounts.buyer.key();
        sale_state.amount = 0;
    }
    assert_owner(sale_state.authority, ctx.accounts.buyer.key())?;

    // Check mint account is allowed
    let mint = ctx.accounts.mint.key();

    // Transfer tokens from user to vault
    let is_native = mint == spl_token::native_mint::id();

    if is_native {
        invoke(
            &system_instruction::transfer(
                &ctx.accounts.token_account.key(),
                &ctx.accounts.vault_account.key(),
                token_amount,
            ),
            &[
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.vault_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    } else {
        assert_is_ata(
            &ctx.accounts.token_account.to_account_info(),
            &ctx.accounts.buyer.key(),
            &ctx.accounts.mint.key(),
        )?;
        invoke(
            &spl_token::instruction::transfer(
                ctx.accounts.token_program.key,
                &ctx.accounts.token_account.key(),
                &ctx.accounts.vault_account.key(),
                &ctx.accounts.buyer.key(),
                &[],
                token_amount,
            )?,
            &[
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.vault_account.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.buyer.to_account_info(),
            ],
        )?;
    }

    // Insert or update sale state for wallet
    sale_state.amount = sale_state
        .amount
        .checked_add(amount)
        .ok_or(SolmateError::InvalidAmount)?;
    // Log presale detail
    msg!(
        "{{\"token_amount\": \"{}\", \"amount\": \"{}\"}}",
        token_amount,
        amount
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Buy<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        seeds = [PREFIX],
        bump,
        has_one = authority
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init_if_needed,
        seeds = [PREFIX, buyer.key().as_ref(), SALE],
        bump,
        space = 8 + std::mem::size_of::<SaleState>(),
        payer = buyer,
    )]
    pub sale_state: Box<Account<'info, SaleState>>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: token account for buyer
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    /// CHECK: token account for vault
    #[account(mut)]
    pub vault_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
