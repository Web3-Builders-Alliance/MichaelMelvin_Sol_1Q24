use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    },
};

use crate::{AmmError, Config};

use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        token::mint = mint_x,
        token::authority = user,
    )]
    user_x_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint_y,
        token::authority = user,
    )]
    user_y_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
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

impl<'info> Deposit<'info> {
    pub fn deposit(&self, amount: u64, max_x: u64, max_y: u64, expiration: i64) -> Result<()> {
        require!(self.config.locked == true, AmmError::Locked);
        require!(expiration > Clock::get()?.unix_timestamp, AmmError::Expired);
        require!(
            amount > 0 && max_x > 0 && max_y > 0,
            AmmError::InvalidAmount
        );

        let (x, y) = match self.mint_lp.supply == 0 && self.vault_x.amount == 0 {
            true => (max_x, max_y),
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    amount,
                    6,
                )
                .map_err(|_| AmmError::InvalidDeposit)?;

                (amounts.x, amounts.y)
            }
        };

        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.user_x_ata.to_account_info(),
            to: self.vault_x.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.mint_x.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, x, self.mint_x.decimals)?;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.user_y_ata.to_account_info(),
            to: self.vault_y.to_account_info(),
            authority: self.user.to_account_info(),
            mint: self.mint_y.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, x, self.mint_y.decimals)?;

        let seeds = &[&b"auth"[..], &[self.config.auth_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_lp_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;

        Ok(())
    }
}
