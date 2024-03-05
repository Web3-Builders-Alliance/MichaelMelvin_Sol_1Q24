use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*;

declare_id!("EQVoqyop6hithNnEi56ozEaibrWTWK7oFozxNH4XhCeQ");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        x_amount_deposited: u64,
        y_amount_expected: u64,
    ) -> Result<()> {
        ctx.accounts
            .make(seed, x_amount_deposited, y_amount_expected)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.cancel()
    }
}
