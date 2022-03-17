use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use super::StakingInstance;
use super::User;
use anchor_spl::token::Mint;
use super::Metadata;
use std::ops::Deref;

#[derive(Accounts)]
#[instruction(
    _staking_instance_bump: u8,
    _staking_user_bump: u8,
)]
pub struct EnterStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
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
    //pub reward_token_mint: AccountInfo<'info>,
    #[account(
        //constraint = nft_token_metadata.mint == nft_token_mint.key(),
    )]
    pub nft_token_mint: Box<Account<'info, Mint>>,
    //pub nft_token_mint: AccountInfo<'info>,
    #[account(
        constraint = nft_token_metadata.owner == &nft_program_id.key(),
    )]
    //pub nft_token_metadata: Box<Account<'info, Metadata>>,
    pub nft_token_metadata: AccountInfo<'info>, 
    #[account(
        constraint = nft_token_authority_wallet
         .clone().into_inner().deref().owner == authority.key(),
        constraint = nft_token_authority_wallet
        .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>,
    //pub nft_token_authority_wallet: AccountInfo<'info>,
    #[account(
        constraint = nft_token_program_wallet
        .clone().into_inner().deref().owner == staking_instance.key(),
        constraint = nft_token_program_wallet
        .clone().into_inner().deref().mint == nft_token_mint.key(),
    )]
    pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>,
    //pub nft_token_program_wallet: AccountInfo<'info>,
    #[account(
        mut, 
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = _staking_instance_bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    //pub staking_instance: AccountInfo<'info>,
    #[account(
        mut, 
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
    )]
    pub user_instance: Account<'info, User>,
    //pub user_instance: AccountInfo<'info>,
    #[account(
        constraint = allowed_collection_address.key() 
            == staking_instance.allowed_collection_address,
    )]
    pub allowed_collection_address: AccountInfo<'info>,
    //#[account(
    //    constraint = 
    //        token_program.key() == Pubkey::new(crate::TOKEN_PROGRAM_BYTES),
    //)]
    pub token_program: AccountInfo<'info>,
    //#[account(
    //    constraint = nft_program_id.key() == Pubkey::new(crate::NFT_TOKEN_PROGRAM_BYTES),
    //)]
    pub nft_program_id: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,

}
