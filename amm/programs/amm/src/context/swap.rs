use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{AmmError, Config};

#[derive(Accounts)]
pub struct Swap<'info> {
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
        mut,
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

impl<'info> Swap<'info> {
    pub fn swap(&self, amount_in: u64, min_out: u64, expiration: i64, is_x: bool) -> Result<()> {
        require!(self.config.locked == true, AmmError::Locked);
        require!(expiration > Clock::get()?.unix_timestamp, AmmError::Expired);
        require!(amount_in > 0, AmmError::InvalidAmount);

        let mut curve = ConstantProduct::init(
            amount_in,
            min_out,
            self.mint_lp.supply,
            self.config.fee,
            Some(6),
        )
        .map_err(|_| AmmError::IncorrectK)?;

        let p = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let result = curve
            .swap(p, amount_in, min_out)
            .map_err(|_| AmmError::IncorrectK)?;

        require!(result.deposit > 0, AmmError::InvalidDeposit);
        require!(result.withdraw > 0, AmmError::InvalidWithdraw);

        self.deposit_tokens(result.deposit, is_x)?;
        self.withdraw_tokens(result.withdraw, is_x)
    }

    fn withdraw_tokens(&self, amount: u64, is_x: bool) -> Result<()> {
        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];

        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_y.to_account_info(),
                self.user_y_ata.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
            false => (
                self.vault_x.to_account_info(),
                self.user_x_ata.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_context, amount, decimals)
    }

    fn deposit_tokens(&self, amount: u64, is_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_x_ata.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.user_y_ata.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_context, amount, decimals)
    }
}
