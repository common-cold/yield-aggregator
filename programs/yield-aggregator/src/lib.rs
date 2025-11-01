use anchor_lang::prelude::*;

declare_id!("GkTQzxKdsb6aCiwDjbqt8ez6kRsfGKXsNHT8hqxMJGsm");

pub mod instructions;
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
}

#[derive(Accounts)]
pub struct Initialize {}