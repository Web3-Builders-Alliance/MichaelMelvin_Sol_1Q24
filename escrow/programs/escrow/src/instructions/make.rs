use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Make<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        payer = maker,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Escrow::INIT_SPACE,
    )]
    pub escrow: Account<'info, Escrow>,

    pub x_mint: InterfaceAccount<'info, Mint>,

    pub y_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
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

impl<'info> Make<'info> {
    pub fn make(
        &mut self,
        seed: u64,
        x_amount_deposited: u64,
        y_amount_expected: u64,
    ) -> Result<()> {
        self.set_escrow(
            seed,
            x_amount_deposited,
            y_amount_expected,
            self.escrow.bump,
        )?;

        self.transfer_tokens(x_amount_deposited)
    }

    fn set_escrow(
        &mut self,
        seed: u64,
        x_amount_deposited: u64,
        y_amount_expected: u64,
        bump: u8,
    ) -> Result<()> {
        let escrow = &mut self.escrow;
        escrow.set_inner(Escrow {
            maker: self.maker.to_account_info().key(),
            x_mint: self.x_mint.to_account_info().key(),
            y_mint: self.y_mint.to_account_info().key(),
            x_amount_deposited,
            y_amount_expected,
            seed,
            bump,
        });

        Ok(())
    }

    fn transfer_tokens(&mut self, x_amount_deposited: u64) -> Result<()> {
        // move the tokens from maker to escrow
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.maker_x_token_account.to_account_info(),
            to: self.escrow.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.x_mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, x_amount_deposited, self.x_mint.decimals)
    }
}
