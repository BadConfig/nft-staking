use anchor_lang::prelude::*;
pub mod cancel_staking;
pub mod enter_staking;
pub mod claim_rewards;
pub mod initialize_staking;
pub mod initialize_user;

// structures from metaverse code of nft metadata
// structures here fully copied from metaverse solana library repo
// difference is anchor resealization derives change
#[account]
//#[derive(AnchorDeserialize)]
pub struct Metadata {
    pub key: Key,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: Data,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable, default is not
    pub is_mutable: bool,
    //// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    //// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<TokenStandard>,
    //// Collection
    pub collection: Option<Collection>,
    //// Uses
    pub uses: Option<Uses>,
}


#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub struct Data {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    /// Array of creators, optional
    pub creators: Option<Vec<Creator>>,
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    // In percentages, NOT basis points ;) Watch out!
    pub share: u8,
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy)]
pub enum Key {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone,Copy)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub struct Uses { // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64, //8
    pub total: u64, //8
}

#[derive(AnchorSerialize,AnchorDeserialize,Clone)]
pub enum TokenStandard {
    NonFungible,  // This is a master edition
    FungibleAsset, // A token with metadata that can also have attrributes
    Fungible,     // A token with simple metadata
    NonFungibleEdition,      // This is a limited edition
}

// staking structures

#[account]
#[derive(Copy,Default)]     
pub struct StakingInstance {
    pub authority: Pubkey,
    pub reward_token_per_sec: u64,
    pub reward_token_mint: Pubkey,
    /// this address is being checked as a verified creator of nft
    pub allowed_collection_address: Pubkey,
    pub accumulated_reward_per_share: u64,
    pub last_reward_timestamp: u64,
    pub total_shares: u64,
}

#[account]
#[derive(Copy,Default)]     
pub struct User {
    pub deposited_amount: u64,
    pub reward_debt: u64,
    pub accumulated_reward: u64,
}

#[cfg(test)]
pub mod test {
    use mpl_token_metadata::state::Metadata;

    #[test]
    fn a() {
        println!("{}",core::mem::size_of::<Metadata>());
    }
}
