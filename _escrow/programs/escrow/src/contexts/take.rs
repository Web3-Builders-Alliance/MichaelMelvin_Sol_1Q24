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
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        close = maker,
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub mint_x: InterfaceAccount<'info, Mint>,

    pub mint_y: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = escrow,
    )]
    pub escrow_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_x,
        associated_token::authority = taker,
    )]
    pub taker_ata_x: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = taker,
    )]
    pub taker_ata_y: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    pub maker_ata_y: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()> {
        self.transfer_from_taker_to_maker()?;
        self.transfer_from_escrow_to_taker()?;
        self.close_escrow()
    }

    fn transfer_from_taker_to_maker(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_y.to_account_info(),
            to: self.maker_ata_y.to_account_info(),
            authority: self.taker.to_account_info(),
            mint: self.mint_y.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.amount_y, self.mint_y.decimals)
    }

    fn transfer_from_escrow_to_taker(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.escrow_ata_x.to_account_info(),
            to: self.taker_ata_x.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_x.to_account_info(),
        };

        let maker_key = self.maker.to_account_info().key();
        let seed_bytes = self.escrow.seed.to_le_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            maker_key.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.escrow.amount_x, self.mint_x.decimals)
    }

    fn close_escrow(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = CloseAccount {
            account: self.escrow_ata_x.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let maker_key = self.maker.to_account_info().key();
        let seed_bytes = self.escrow.seed.to_le_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            maker_key.as_ref(),
            seed_bytes.as_ref(),
            &[self.escrow.bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)
    }
}
