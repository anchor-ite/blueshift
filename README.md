# Blueshift - Solana/Anchor Learning Journey

Solana/Anchor learning repo

## Programs

All the programs were built using Anchor (and anchor-spl) v1.0.0-rc.2
Key findings about this version:
- For the programs' ID in Cpi calls the compiler now expects 'Pubkey'.key()
 instead of AccountInfo i.e. to_account_info().
- Except for the above, the Anchor code for these programs is the same than
for v0.32.1

### anchor-vault challenge

**Status**: All Blueshift tests passing
**Anchor Version**: v1.0.0-rc.2

### anchor-escrow program

**Status**: All Blueshift tests passing
**Anchor Version**: v1.0.0-rc.2 
**Tag**: `anchor-1.0.0-rc.2`

### anchor-amm

Work in progress

### Stack
- Solana
- Anchor Framework
- Rust
- TypeScrypt
- Surfpool
