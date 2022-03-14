use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use super::StakingInstance;
use super::User;
use anchor_spl::token::Mint;
use super::Metadata;

#[derive(Accounts)]
#[instruction(
    _staking_instance_bump: u8,
    _staking_user_bump: u8,
    _metadata_instance_bump: u8,
)]
pub struct EnterStaking<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap()
            .key()
            .eq(
                &staking_instance.key(),
            )
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = nft_token_metadata.mint == nft_token_mint.key(),
    )]
    pub nft_token_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [
            b"metadata".as_ref(), 
            Pubkey::new(crate::NFT_TOKEN_PROGRAM_BYTES).as_ref(),
            nft_token_mint.key().as_ref()
        ],
        bump = _metadata_instance_bump,
        constraint = nft_token_metadata
            .data
            .creators
            .as_ref()
            .unwrap()
            .iter()
            .filter(|item|{
                allowed_collection_address.key() == item.address && item.verified
            })
            .count() > 0,
    )]
    pub nft_token_metadata: Box<Account<'info, Metadata>>,
    #[account(
        mut,
        associated_token::mint = nft_token_mint,
        associated_token::authority = authority,
    )]
    pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = nft_token_mint,
        associated_token::authority = staking_instance,
    )]
    pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
        mut, 
        seeds = [crate::STAKING_SEED.as_ref(),staking_instance.authority.as_ref()],
        bump = _staking_instance_bump,
    )]
    pub staking_instance: Box<Account<'info, StakingInstance>>,
    #[account(
        init, 
        seeds = [
            crate::USER_SEED.as_ref(),
            staking_instance.key().as_ref(),
            authority.key().as_ref()
        ],
        bump = _staking_user_bump,
        //space = 8 + core::mem::size_of::<User>(),
        payer = authority,
    )]
    pub user_instance: Box<Account<'info, User>>,
    #[account(
        constraint = allowed_collection_address.key() 
            == staking_instance.allowed_collection_address,
    )]
    pub allowed_collection_address: AccountInfo<'info>,
    #[account(
        constraint = 
            token_program.key() == Pubkey::new(crate::TOKEN_PROGRAM_BYTES),
    )]
    pub token_program: AccountInfo<'info>,
    #[account(
        constraint = 
            token_program.key() == Pubkey::new(crate::NFT_TOKEN_PROGRAM_BYTES),
    )]
    pub nft_program_id: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,

}

