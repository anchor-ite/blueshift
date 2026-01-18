use anchor_lang::prelude::*;

declare_id!("GtNPFLejnMtyNqxxUnwF5Bi4ns2vzc3ac6Gm1gHdsvV5");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
