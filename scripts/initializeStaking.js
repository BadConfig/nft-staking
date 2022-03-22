import anchor from '@project-serum/anchor'
import {
    Connection,
    clusterApiUrl,
    PublicKey,
} from '@solana/web3.js'
import staking from '../nft_staking.json'
import WALLET from './wallet.json';

import {programID, candyV2, rewardToken, opts, seeds} from "./common.js"
const { SystemProgram, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY } = anchor.web3

async function main() {
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
            wallet.publicKey.toBuffer()], programID);

   await program.rpc.initializeStaking(new anchor.BN(10*9), programBump, {
       accounts: {
           authority: wallet.publicKey,
           rewardTokenMint: rewardToken,
           allowedCollectionAddress: candyV2,
           stakingInstance: programPDA,
           systemProgram: SystemProgram.programId,
           rent: SYSVAR_RENT_PUBKEY,
           time: SYSVAR_CLOCK_PUBKEY
       },
       signers: [wallet],
   })

}

main().then("finish")

// node --experimental-json-modules initializeStaking.js