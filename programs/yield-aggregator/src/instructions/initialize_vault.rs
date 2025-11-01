use anchor_lang::prelude::*;
use anchor_lang::{account, InitSpace, Key, Result, Space, ToAccountInfo};

use anchor_spl::token_interface::{Mint, TokenAccount};
use anchor_spl::token_2022::{transfer_checked, Token2022, TransferChecked};
use anchor_spl::{associated_token::AssociatedToken};

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub owner_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + VaultAccount::INIT_SPACE,
        seeds = [b"vault".as_ref(), owner.key().as_ref()],
        bump
    )]
    pub vault_account: Account<'info, VaultAccount>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = usdc_mint,
        associated_token::authority = vault_account,
        associated_token::token_program = token_program
    )]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,


    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>
    
}

#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub owner: Pubkey,
    pub total_usdc_deposited: u64,
    pub allocation_jup_lend_bps: u16,
    pub allocation_kamino_bps: u16,
    pub last_allocation_ts: i64,
    pub bump: u8
}


impl<'info> InitializeVault<'info> {
    pub fn process(&mut self, amount: u64, vault_account_bump: u8) -> Result<()> {
        
        //transfer usdc from user's ata to vault's usdc ata
        let transfer_ctx = CpiContext::new(
            self.token_program.to_account_info(), 
            TransferChecked {
                from: self.owner_usdc_ata.to_account_info(),
                mint: self.usdc_mint.to_account_info(),
                to: self.vault_usdc_ata.to_account_info(),
                authority: self.owner.to_account_info()
            }
        );
        transfer_checked(
            transfer_ctx, 
            amount, 
            6
        )?;

        //add data in vault account
        let vault_account = &mut self.vault_account;
        vault_account.owner = self.owner.key();
        vault_account.total_usdc_deposited = amount;
        vault_account.allocation_jup_lend_bps  = 0;
        vault_account.allocation_kamino_bps = 0;
        vault_account.last_allocation_ts = Clock::get()?.unix_timestamp;
        vault_account.bump = vault_account_bump;

        Ok(())
    }
}


