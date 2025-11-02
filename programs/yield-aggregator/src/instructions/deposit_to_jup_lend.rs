use anchor_lang::prelude::*;
use anchor_lang::{account, InitSpace, Key, Result, Space, ToAccountInfo};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::cpi::jupiter::deposit::DepositParams;


#[derive(Accounts)]
pub struct DepositToJuplend<'info> {
    // User accounts
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(mut)]
    pub depositor_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = f_token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    // Protocol accounts
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub lending_admin: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub lending: AccountInfo<'info>,

    #[account(mut)]
    pub f_token_mint: InterfaceAccount<'info, Mint>,

    // Liquidity protocol accounts

    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub supply_token_reserves_liquidity: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub lending_supply_position_on_liquidity: AccountInfo<'info>,
    
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub rate_model: AccountInfo<'info>,
    
    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub vault: AccountInfo<'info>,
    
    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub liquidity: AccountInfo<'info>,
    
    #[account(mut)]
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub liquidity_program: AccountInfo<'info>,

    // Rewards and programs
    
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub rewards_rate_model: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    

    // Target lending program
    /// CHECK: We don’t need to deserialize this account; we just pass it to the CPI
    pub lending_program: UncheckedAccount<'info>,
}

impl<'info> DepositToJuplend<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let deposit_params = DepositParams {
            signer: self.signer.to_account_info(),
            depositor_token_account: self.depositor_token_account.to_account_info(),
            recipient_token_account: self.recipient_token_account.to_account_info(),
            mint: self.mint.to_account_info(),
            lending_admin: self.lending_admin.to_account_info(),
            lending: self.lending.to_account_info(),
            f_token_mint: self.f_token_mint.to_account_info(),
            supply_token_reserves_liquidity: self.supply_token_reserves_liquidity.to_account_info(),
            lending_supply_position_on_liquidity: self.lending_supply_position_on_liquidity.to_account_info(),
            rate_model: self.rate_model.to_account_info(),
            vault: self.vault.to_account_info(),
            liquidity: self.liquidity.to_account_info(),
            liquidity_program: self.liquidity_program.to_account_info(),
            rewards_rate_model: self.rewards_rate_model.to_account_info(),
            token_program: self.token_program.to_account_info(),
            associated_token_program: self.associated_token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            lending_program: self.lending_program.clone(),
        };
        deposit_params.deposit(amount)?;
        Ok(())
    }
}