mod utils;

#[cfg(feature = "test-bpf")]
mod buy {
    use crate::{
        setup_context,
        utils::{
            helpers::{airdrop, create_mint, create_token_account, mint_to},
        },
    };
    use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
    use solana_program::clock::Clock;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer,
        system_program, sysvar, transaction::Transaction, transport::TransportError,
    };

    #[tokio::test]
    async fn success() {
        setup_context!(context, mpl_token_metadata);

        let god = Keypair::new();

        airdrop(
            &mut context,
            &god.pubkey(),
            10_000_000_000,
        )
        .await;

        let mint_key = Keypair::new();

        create_mint(
            &mut context,
            &mint_key,
            &god.pubkey(),
            0,
        )
        .await;

        create_token_account(
            &mut context,
            &god,
            &mint_key.pubkey(),
            &god.pubkey(),
        )
        .await;
    }
}
