import anchor from '@project-serum/anchor'
import {
    Connection,
    clusterApiUrl,
    PublicKey,
} from '@solana/web3.js'
import { TOKEN_PROGRAM_ID } from "@solana/spl-token"
import staking from '../nft_staking.json'
import WALLET from './owner.json';
import dotenv from 'dotenv'

import { 
    programID, 
    tokenMetadata, 
    rewardToken, 
    opts, 
    seeds, 
    owner, 
    getOrCreateAssociatedTokenAccount 
} from "./common.js"
const { SystemProgram, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY } = anchor.web3

async function main() {
    const conf = (dotenv.config()).parsed
    const nftMint = new PublicKey(conf.NFT_MINT)

    const network = clusterApiUrl("devnet")
    const connection = new Connection(network, opts.preflightCommitment);
    const wallet = anchor.web3.Keypair.fromSecretKey(Uint8Array.from(WALLET))
    const walletK = new anchor.Wallet(wallet)
    const provider = new anchor.Provider(
        connection, walletK, opts.preflightCommitment,
    )
    const program = new anchor.Program(staking, programID, provider);

    const [programPDA, programBump] =
        await PublicKey.findProgramAddress([Buffer.from(seeds.staking, "utf8"),
        owner.toBuffer()], programID);

    const [userPDA, userBump] = await PublicKey.findProgramAddress(
        [Buffer.from(seeds.user), programPDA.toBuffer(), wallet.publicKey.toBuffer()], programID
    );

    const userRewardATA = await getOrCreateAssociatedTokenAccount(
        wallet.publicKey,
        rewardToken
    );
    
    const BN = anchor.BN;
    let txn = await program.rpc.claimRewards(
        new BN(0),// 0 - claim all
        new BN(programBump),
        new BN(userBump), {
         accounts: {
             authority: wallet.publicKey,
             rewardTokenMint: rewardToken,
             rewardTokenAuthorityWallet: userRewardATA,
             userInstance: userPDA,
             stakingInstance: programPDA,
             tokenProgram: TOKEN_PROGRAM_ID,
             systemProgram: SystemProgram.programId,
             rent: SYSVAR_RENT_PUBKEY,
             time: SYSVAR_CLOCK_PUBKEY
         },
         signers: [wallet],
       }
    )

    console.log(txn)
    // const acc = await program.account.myAccount.fetch(localAccount.publicKey)

}

main().then("finish")

// node --experimental-json-modules claimReward.js