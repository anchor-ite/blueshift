use anchor_lang::prelude::*;

declare_id!("9rMJ2sb6uwqtzjH7vy4w3YykZCSJdrmq6Big9LTUmcpi");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
