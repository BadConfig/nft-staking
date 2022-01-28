use anchor_lang::prelude::*;
use super::StakingInstance;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(
    token_per_sec: u64,
    staking_instance_bump: u8,
)]
pub struct InitializeStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        // reward token mint ownership constraint may go here
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,
    #[account(
        init, 
        //seeds = [b"stakingInstance".as_ref()],
        //bump = signersignerstaking_instance_bump,
        space = 8 + core::mem::size_of::<StakingInstance>(),

        payer = authority,
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    pub allowed_collection_address: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,
    pub token_program: AccountInfo<'info>,

}

