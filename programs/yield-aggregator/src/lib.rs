use anchor_lang::prelude::*;

declare_id!("Ciqbm5prTp5nF8qGB3265qjcwZdtxQzyYSYsYWCfp8qV");

pub mod instructions;
pub mod cpi;
use crate::instructions::*;

#[program]
pub mod yield_aggregator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn initialize_vault<'info> (ctx: Context<'_, '_, '_, 'info, InitializeVault<'info>>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount, ctx.bumps.vault_account)
    }

    pub fn deposit_to_jup_lend<'info> (ctx: Context<'_, '_, '_, 'info, DepositToJuplend<'info>>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}