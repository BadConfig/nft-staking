use anchor_lang::prelude::*;
use super::{
    StakingInstance,
    User,
};

#[derive(Accounts)]
#[instruction(
    _staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init, 
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
        payer = authority,
    )]
    pub user_instance: Box<Account<'info, User>>,
    #[account(
        mut, 
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = _staking_instance_bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,
}

