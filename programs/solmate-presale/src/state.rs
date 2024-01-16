use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub authority: Pubkey,
    pub start_date: u64,
    pub end_date: u64,
    pub activated: bool,
}

#[account]
#[derive(Default)]
pub struct SaleState {
    pub authority: Pubkey,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Copy)]
pub struct PresaleMint {
    pub mint: Pubkey,
    pub decimals: u64,
    pub price: u64,
}
