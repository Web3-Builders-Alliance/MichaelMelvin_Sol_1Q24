use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{AmmError, Config};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Init<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        space = Config::INIT_SPACE,
        seeds = [b"config", seed.to_le_bytes().as_ref(), mint_x.key().as_ref(), mint_y.key().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = auth,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        seeds = [b"auth"],
        bump,
    )]
    /// CHECK: checked via constraints
    pub auth: UncheckedAccount<'info>,

    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = initializer,
        token::mint = mint_x,
        token::authority = config,
    )]
    pub vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = initializer,
        token::mint = mint_y,
        token::authority = config,
    )]
    pub vault_y: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Init<'info> {
    pub fn init(&mut self, seed: u64, fee: u16, bumps: &InitBumps) -> Result<()> {
        require!(fee <= 10_000, AmmError::FeeTooHigh);

        self.config.set_inner(Config {
            seed,
            authority: self.auth.key(),
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_mint_bump: bumps.mint_lp,
            auth_bump: bumps.auth,
        });

        Ok(())
    }
}
