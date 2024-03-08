use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        burn, transfer_checked, Burn, Mint, TokenAccount, TokenInterface, TransferChecked,
    },
};
use constant_product_curve::ConstantProduct;

use crate::{AmmError, Config};

#[derive(Accounts)]
pub struct Withdaw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"config",
            config.seed.to_le_bytes().as_ref(),
            mint_x.key().as_ref(),
            mint_y.key().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"lp", config.key().as_ref()],
        bump = config.lp_mint_bump,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    user_x_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    user_y_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint_lp,
        token::authority = user,
    )]
    user_lp_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint_x,
        token::authority = config,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint_y,
        token::authority = config,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds = [b"auth"],
        bump,
    )]
    /// CHECK: safe to use unchecked account here
    pub auth: UncheckedAccount<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Withdaw<'info> {
    pub fn withdraw(&mut self, amount: u64, min_x: u64, min_y: u64, expiration: i64) -> Result<()> {
        require!(self.config.locked == true, AmmError::Locked);
        require!(expiration > Clock::get()?.unix_timestamp, AmmError::Expired);
        require!(
            amount > 0 && min_x > 0 && min_y > 0,
            AmmError::InvalidAmount
        );

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            amount,
            6,
        )
        .map_err(|_| AmmError::InvalidWithdraw)?;

        require!(
            amounts.x <= min_x && amounts.y <= min_y,
            AmmError::SlippageExceeded
        );

        self.withdraw_tokens(amount, true)?;
        self.withdraw_tokens(amount, false)?;
        self.burn_lp_tokens(amount)
    }

    fn burn_lp_tokens(&self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_lp_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        burn(cpi_ctx, amount)
    }

    fn withdraw_tokens(&self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_x_ata.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_y_ata.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.auth.to_account_info(),
            mint,
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, amount, decimals)?;

        Ok(())
    }
}
