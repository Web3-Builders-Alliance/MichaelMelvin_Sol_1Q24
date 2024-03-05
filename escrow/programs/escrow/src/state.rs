use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub x_mint: Pubkey,
    pub y_mint: Pubkey,
    pub x_amount_deposited: u64,
    pub y_amount_expected: u64,
    pub seed: u64,
    pub bump: u8,
}

impl Space for Escrow {
    const INIT_SPACE: usize = 8 + 32 + 32 + 32 + 8 + 8 + 8 + 1;
}
