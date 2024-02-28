use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

declare_id!("6PGRZVPfNMhZ5u9tWweKBVifEY313yBHkTyRkfsf7Fxe");

#[program]
pub mod vault {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_state.maker = ctx.accounts.maker.key();
        ctx.accounts.vault_state.taker = ctx.accounts.taker.key();
        ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;
        ctx.accounts.vault_state.vault_bump = ctx.bumps.vault;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.maker.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        system_program::transfer(cpi_ctx, amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        let maker = ctx.accounts.maker.key();
        let vault_bump = ctx.accounts.vault_state.vault_bump;

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", maker.as_ref(), &[vault_bump]]];
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        system_program::transfer(cpi_ctx, ctx.accounts.vault.lamports())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let maker = ctx.accounts.maker.key();
        let vault_bump = ctx.accounts.vault_state.vault_bump;

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", maker.as_ref(), &[vault_bump]]];
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.maker.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        system_program::transfer(cpi_ctx, ctx.accounts.vault.lamports())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init,
        seeds = [b"VaultState", maker.key().as_ref()],
        bump,
        payer = maker,
        space = VaultState::INIT_SPACE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", maker.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub taker: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        has_one = maker,
        seeds = [b"VaultState", maker.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", maker.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [b"VaultState", maker.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", maker.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    pub taker: SystemAccount<'info>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [b"VaultState", maker.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", maker.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    pub maker: Pubkey,
    pub taker: Pubkey,
    pub state_bump: u8,
    pub vault_bump: u8,
}

impl Space for VaultState {
    const INIT_SPACE: usize = 8 + 32 + 32 + 1 + 1;
}
