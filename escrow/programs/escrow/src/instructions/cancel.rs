use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub x_mint: InterfaceAccount<'info, Mint>,

    pub y_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = x_mint,
        associated_token::authority = escrow,
    )]
    pub escrow_x_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = x_mint,
        associated_token::authority = maker,
    )]
    pub maker_x_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> Cancel<'info> {
    pub fn cancel(&mut self) -> Result<()> {
        self.tranfer_from_escrow_to_maker()?;
        self.close_accounts()
    }

    fn tranfer_from_escrow_to_maker(&mut self) -> Result<()> {
        let maker_key = self.maker.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            maker_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.escrow_x_token_account.to_account_info(),
            to: self.maker_x_token_account.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.x_mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(
            cpi_ctx,
            self.escrow_x_token_account.amount,
            self.x_mint.decimals,
        )
    }

    fn close_accounts(&mut self) -> Result<()> {
        let maker_key = self.maker.key();
        let seed = self.escrow.seed.to_le_bytes();

        let seeds: &[&[u8]; 4] = &[
            b"escrow",
            maker_key.as_ref(),
            seed.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.escrow_x_token_account.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)
    }
}
