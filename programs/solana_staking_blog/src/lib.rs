use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo, Transfer};

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("ASgW8pBd9jpdvEcKMS1f34hLBML4NNajDrqjQhr5u7ED");

#[program]
pub mod solana_staking_blog {
    use super::*; // bring all the items from the parent module to current scope

    pub fn initialize(ctx: Context<Initialize>, start_slot: u64, end_slot: u64) -> Result<()> {
        msg!("Initializing the program");
        let pool_info = &mut ctx.accounts.pool_info;
        pool_info.admin = ctx.accounts.admin.key();
        pool_info.start_slot = start_slot;
        pool_info.end_slot = end_slot;
        pool_info.token = ctx.accounts.staking_token.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    //init: Indicate this account should be created and initialized
    //payer: The account that will pay for the creation of this account
    //space: The size of the account in bytes
    //pool_info: will hold an account of type PoolInfo
    //purpose: used within a context struct to specify that an account of
    // type PoolInfo should be initialized and created when the context is executed
    #[account(init, payer = admin, space = 8 + PoolInfo::LEN)]
    pub pool_info: Account<'info, PoolInfo>,

    #[account(mut)]
    pub staking_token: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub admin_staking_wallet: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}



// PoolInfo represent an account in the Solana program, tell anchor to
// treat this struct as an account and generate necessary boilerplate code
// for serialization and deserialization
#[account]
pub struct PoolInfo {
    pub admin: Pubkey,
    pub start_slot: u64,
    pub end_slot: u64,
    pub token: Pubkey,
}

impl PoolInfo {
    const LEN: usize = 32 + 8 + 8 + 32; // Size of PoolInfo struct in bytes
}