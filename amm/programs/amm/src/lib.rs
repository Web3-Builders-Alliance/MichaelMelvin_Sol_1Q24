use anchor_lang::prelude::*;

declare_id!("AjTrriDodhzonGJ3mCp2WknCaYNV58eXKiewD3YyhKjo");

pub mod context;
pub mod errors;
pub mod state;

pub use context::*;
pub use errors::*;
pub use state::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_amm(ctx: Context<Init>, seed: u64, fee: u16) -> Result<()> {
        ctx.accounts.init(seed, fee, &ctx.bumps)
    }
}
