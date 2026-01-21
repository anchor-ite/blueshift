#[derive(Accounts)]
pub struct Take <'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seeds.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker @ EscrowError::InvalidMaker,
        has_one = mint_a @ EscrowError::InvalidMintA,
        has_one = mint_b @ EscrowError::InvalidMintB,
    )]
    pub escrow: Box<Account<'info, Escrow>>,

    //Token accounts
    
    pub mint_a: Box<InterfaceAccount<'info, Mint>>,
    pub mint_b: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenProgram>>,
    
    #[account]
    pub taker_ata_a:
    pub taker_ata_b: 

    pub maker_ata_b

    //Programs
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenProgram>,
    pub system_program: Program<'info, System>,
    

}