use anchor_lang::prelude::*;

pub mod state;

pub mod error;

pub mod constant;

pub mod processor;

pub mod utils;

use crate::processor::*;

declare_id!("HXbR9vT3guUKtip1Z88R3hxRNpEaScV97fwn8nYsL54D");

#[program]
pub mod solmate_presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, start_date: u64, end_date: u64) -> Result<()> {
        initialize::handle(ctx, start_date, end_date)
    }

    pub fn update_setting(
        ctx: Context<UpdateSetting>,
        start_date: Option<u64>,
        end_date: Option<u64>,
        activated: Option<bool>,
    ) -> Result<()> {
        update_setting::handle(ctx, start_date, end_date, activated)
    }

    pub fn buy(ctx: Context<Buy>, token_amount: u64, amount: u64) -> Result<()> {
        buy::handle(ctx, token_amount, amount)
    }
}
