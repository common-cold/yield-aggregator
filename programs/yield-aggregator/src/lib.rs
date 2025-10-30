use anchor_lang::prelude::*;

declare_id!("GkTQzxKdsb6aCiwDjbqt8ez6kRsfGKXsNHT8hqxMJGsm");

pub mod instructions;

#[program]
pub mod yield_aggregator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}