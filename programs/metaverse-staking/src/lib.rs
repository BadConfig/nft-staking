use anchor_lang::prelude::*;
pub mod structures;
use structures::{
    initialize::*,
    enter_staking::*,
    cancel_staking::*,
};

use anchor_spl::token::{
    self, Burn,
    MintTo, Transfer,
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod metaverse_staking {
    use super::*;
    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        token_per_sec: u64,
        staking_instance_bump: u8,
    ) -> ProgramResult {
        let mut staking_instance = &mut ctx.accounts.staking_instance;
        staking_instance.authority= ctx.accounts.authority.key().clone();
        staking_instance.reward_token_mint = ctx
            .accounts
            .reward_token_mint
            .to_account_info()
            .key()
            .clone();
        staking_instance.reward_token_per_sec = token_per_sec;
        staking_instance.last_reward_timestamp = ctx.accounts.time.unix_timestamp as u64;
        staking_instance.accumulated_reward_per_share = 0;
        staking_instance.allowed_collection_address = ctx
            .accounts
            .allowed_collection_address
            .key()
            .clone();
        Ok(())
    }
    pub fn enter_staking(
        ctx: Context<EnterStaking>,
        amount: u64,
        staking_instance_bump: u8,
    ) -> ProgramResult {
        let mut staking_instance = *ctx.accounts.staking_instance;
        let mut user_instance = *ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;

        let income = staking_instance.reward_token_per_sec *
        (current_timestamp - staking_instance.last_reward_timestamp) as u64;
            staking_instance.accumulated_reward_per_share += 
                income * 10u64.pow(10) / staking_instance.total_shares;
        staking_instance.last_reward_timestamp = current_timestamp;


        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_program_wallet.to_account_info(),
            from: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(context, amount)?;

        user_instance.deposited_amount += amount;
        staking_instance.total_shares += amount;
        user_instance.reward_debt = 
            amount * staking_instance.accumulated_reward_per_share / 10u64.pow(10);

        Ok(())
    }

    pub fn cancel_staking(
        ctx: Context<CancelStaking>,
        amount: u64,
        staking_instance_bump: u8,
    ) -> ProgramResult {
        let mut staking_instance = *ctx.accounts.staking_instance;
        let mut user_instance = *ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;

        let income = staking_instance.reward_token_per_sec *
        (current_timestamp - staking_instance.last_reward_timestamp) as u64;
            staking_instance.accumulated_reward_per_share += 
                income * 10u64.pow(10) / staking_instance.total_shares;
        staking_instance.last_reward_timestamp = current_timestamp;

        let pending_reward = (user_instance.deposited_amount * 
            staking_instance.accumulated_reward_per_share) / 10u64.pow(12) 
            - user_instance.reward_debt;

        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            to: ctx.accounts.reward_token_authority_wallet.to_account_info(),
            authority: ctx.accounts.staking_instance.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(context, pending_reward)?;


        user_instance.deposited_amount -= amount;
        staking_instance.total_shares -= amount;
        user_instance.reward_debt = 
            amount * staking_instance.accumulated_reward_per_share / 10u64.pow(10);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
