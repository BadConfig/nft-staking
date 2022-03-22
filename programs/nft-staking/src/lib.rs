pub mod structures;
pub mod constants;

use anchor_lang::prelude::*;
use constants::*;
use structures::{
    initialize_user::*,
    initialize_staking::*,
    enter_staking::*,
    cancel_staking::*,
    claim_rewards::*,
    StakingInstance,
    User,
};
use anchor_spl::token::{
    self,
    MintTo, 
    Transfer,
};



declare_id!("EXZJ3cCRD8dxmTYPxY4nWEWjGiwiW1U9yz2jzJzngnLg");

fn update_reward_pool(
    current_timestamp: u64,
    staking_instance: &mut StakingInstance,
    #[allow(unused_variables)]
    user_instance: &mut User,
) {
    let income = staking_instance.reward_token_per_sec
        .checked_mul(current_timestamp
        .checked_sub(staking_instance.last_reward_timestamp)
        .unwrap())
        .unwrap();
    staking_instance.accumulated_reward_per_share = 
        staking_instance.accumulated_reward_per_share
        .checked_add(income.checked_mul(COMPUTATION_DECIMALS).unwrap()
        .checked_div(staking_instance.total_shares)
        .unwrap_or(0))
        .unwrap();
    staking_instance.last_reward_timestamp = current_timestamp;
}

fn store_pending_reward(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
) {
    user_instance.accumulated_reward = user_instance.accumulated_reward
        .checked_add(user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap()
        .checked_sub(user_instance.reward_debt)
        .unwrap())
        .unwrap();
}

fn update_reward_debt(
    staking_instance: &mut StakingInstance,
    user_instance: &mut User,
) {
    user_instance.reward_debt = user_instance.deposited_amount
        .checked_mul(staking_instance.accumulated_reward_per_share)
        .unwrap()
        .checked_div(COMPUTATION_DECIMALS)
        .unwrap();
}

#[program]
pub mod nft_staking {
    use super::*;
    pub fn initialize_staking(
        ctx: Context<InitializeStaking>,
        token_per_sec: u64,
        _staking_instance_bump: u8,
    ) -> ProgramResult {
        let staking_instance = &mut ctx.accounts.staking_instance;
        staking_instance.authority= ctx.accounts.authority.key().clone();
        staking_instance.reward_token_per_sec = token_per_sec;
        staking_instance.last_reward_timestamp = ctx.accounts.time.unix_timestamp as u64;
        staking_instance.accumulated_reward_per_share = 0;
        staking_instance.reward_token_mint = ctx
            .accounts
            .reward_token_mint
            .to_account_info()
            .key()
            .clone();
        staking_instance.allowed_collection_address = ctx
            .accounts
            .allowed_collection_address
            .key()
            .clone();
        Ok(())
    }

    pub fn initialize_user(
        ctx: Context<InitializeUser>,
        _staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> ProgramResult {
        let user_instance = &mut ctx.accounts.user_instance;
        user_instance.deposited_amount = 0;
        user_instance.reward_debt = 0;
        user_instance.accumulated_reward = 0;
        Ok(())
    }

    pub fn enter_staking(
        ctx: Context<EnterStaking>,
        _staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() == 
                    item.address && item.verified
            })
            .count() == 0;
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }
        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );

        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_program_wallet.to_account_info(),
            from: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(context, 1)?;

        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_add(1)
            .unwrap();
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_add(1)
            .unwrap();
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }


    pub fn cancel_staking(
        ctx: Context<CancelStaking>,
        staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.nft_token_metadata.try_borrow_data()?;
        msg!("borrow");
        let val = mpl_token_metadata::state::Metadata::deserialize(&mut &data[..])?;
        msg!("deser");
        let collection_not_proper = val
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                ctx.accounts.allowed_collection_address.key() == 
                    item.address && item.verified
            })
            .count() == 0;
        msg!("count");
        if collection_not_proper || val.mint != ctx.accounts.nft_token_mint.key() {
            msg!("error");
            return Ok(());
        }

        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        msg!("get accounts");
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        msg!("upd pool");
        store_pending_reward(
            staking_instance,
            user_instance,
        );

        let cpi_accounts = Transfer {
            to: ctx.accounts.nft_token_authority_wallet.to_account_info(),
            from: ctx.accounts.nft_token_program_wallet.to_account_info(),
            authority: staking_instance.clone().to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[
            &STAKING_SEED[..], 
            staking_instance.authority.as_ref(), 
            &[staking_instance_bump]
        ];
        token::transfer(context.with_signer(&[&authority_seeds[..]]), 1)?;

        user_instance.deposited_amount = user_instance
            .deposited_amount
            .checked_sub(1)
            .unwrap();
        staking_instance.total_shares = staking_instance
            .total_shares
            .checked_sub(1)
            .unwrap();
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }

    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        amount: u64,
        staking_instance_bump: u8,
        _staking_user_bump: u8,
    ) -> ProgramResult {
        let staking_instance = &mut ctx.accounts.staking_instance;
        let user_instance = &mut ctx.accounts.user_instance;
        let current_timestamp = ctx.accounts.time.unix_timestamp as u64;
        update_reward_pool(
            current_timestamp,
            staking_instance,
            user_instance,
        );
        store_pending_reward(
            staking_instance,
            user_instance,
        );

        let cpi_accounts = MintTo {
            mint: ctx.accounts.reward_token_mint.to_account_info(),
            to: ctx.accounts.reward_token_authority_wallet.to_account_info(),
            authority: staking_instance.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let context = CpiContext::new(cpi_program, cpi_accounts);
        let authority_seeds = &[
            &STAKING_SEED[..], 
            staking_instance.authority.as_ref(), 
            &[staking_instance_bump]
        ];

        let amount = if amount == 0 {
            user_instance.accumulated_reward
        } else {
            amount
        };
        user_instance.accumulated_reward = user_instance
            .accumulated_reward
            .checked_sub(amount)
            .unwrap();

        token::mint_to(context.with_signer(&[&authority_seeds[..]]), amount)?;
        update_reward_debt(
            staking_instance,
            user_instance,
        );
        Ok(())
    }
}


