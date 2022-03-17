use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use super::StakingInstance;
use super::User;
use anchor_spl::token::Mint;

#[derive(Accounts)]
#[instruction(
    amount: u64,
    staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct ClaimRewards<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap()
            .eq(
                &staking_instance.key(),
            )
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = reward_token_mint,
        associated_token::authority = authority,
    )]
    pub reward_token_authority_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
        mut, 
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = staking_instance_bump,
    )]
    pub staking_instance: Box<Account<'info, StakingInstance>>,
    #[account(
        mut, 
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
    )]
    pub user_instance: Box<Account<'info, User>>,
    #[account(
        constraint = 
            token_program.key() == crate::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,

}

