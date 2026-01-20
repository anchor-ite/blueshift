use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer}
};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
        transfer(
            CpiContext::new(
//Unlike previous Anchor versions, in v1.0.0-rc.2 the compiler expects 'Pubkey'
//instead of 'AccountInfo<'_>' thus, .key() instead of to_account_info()
                ctx.accounts.system_program.key(), 
                Transfer {
                    from: ctx.accounts.signer.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        require_neq!(ctx.accounts.vault.lamports(), 0, VaultError::InvalidAmount);
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];

        transfer(
            CpiContext::new_with_signer(
//Unlike previous Anchor versions, in v1.0.0-rc.2 the compiler expects 'Pubkey'
//instead of 'AccountInfo<'_>' thus, .key() instead of to_account_info()
                ctx.accounts.system_program.key(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.signer.to_account_info(),
                },
                &[&signer_seeds[..]]
            ),
            ctx.accounts.vault.lamports()
        )?;
        Ok(())
    }
}


#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Vault already exists")]
    VaultAlreadyExists,
    #[msg("Invalid amount")]
    InvalidAmount,
}
