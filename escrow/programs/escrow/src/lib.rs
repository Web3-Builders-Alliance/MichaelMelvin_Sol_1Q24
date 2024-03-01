use anchor_lang::prelude::*;

declare_id!("5jd71oZ4bDw53bsVDCfPieohbHutweorNXMADt7CLNKZ");

pub mod contexts;
pub mod state;

pub use contexts::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount_x: u64, amount_y: u64) -> Result<()> {
        ctx.accounts
            .make(seed, amount_x, amount_y, &MakeBumps::from(ctx.bumps))
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
