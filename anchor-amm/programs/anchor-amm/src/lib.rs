use anchor_lang::prelude::*;

declare_id!("AXW5x9BWApLdA2193r2PZJBokco8x1NvrQzYJX4eGt1y");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
