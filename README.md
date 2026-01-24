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

### anchor-flashloan
The program is compatible with Anchor v1.0.0-rc.2.

### Breaking Changes Addressed

#### 1. CPI Context Program IDs
Changed from `.to_account_info()` to `.key()` in all lines involving programID in CPI calls.

**Files changed:**
- `programs/*/src/lib.rs` - All `CpiContext::new()` and `CpiContext::new_with_signer()` calls

**Example:**
```diff
  transfer(
      CpiContext::new(
-         ctx.accounts.token_program.to_account_info(),
+         ctx.accounts.token_program.key(),
          Transfer { ... }
      ),
      amount
  )?;
```

#### 2. Instruction Introspection Imports
Updated to import from `solana-instructions-sysvar` crate directly.

**Files changed:**
- `programs/anchor-flashloan/Cargo.toml` - Added dependency
- `programs/anchor-flashloan/src/lib.rs` - Updated imports

**Changes:**
```diff
- use anchor_lang::solana_program::sysvar::instructions::{...};
+ use solana_instructions_sysvar::{
+     ID as INSTRUCTIONS_SYSVAR_ID,
+     load_instruction_at_checked,
+     load_current_index_checked,
+ };
```

**New dependency:**
```toml
solana-instructions-sysvar = "3.0.0"
```

### Rationale
Anchor v1.0.0-rc.2 uses Solana SDK v3.x, which modularized the codebase. 
Anchor no longer re-exports all `solana_program` utilities, requiring 
direct imports from specialized crates.

### Testing
All programs compile successfully
All Blueshift tests pass
Work in progress

### Stack
- Solana
- Anchor Framework
- Rust
- TypeScrypt
- Surfpool
