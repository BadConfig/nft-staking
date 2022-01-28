import {
	PublicKey,
	SystemProgram,
  Transaction,
} from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";

import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MetaverseStaking } from '../target/types/metaverse_staking';

anchor.setProvider(anchor.Provider.env());



export const provider = anchor.Provider.env();

export const payer = anchor.web3.Keypair.generate();
export const user = anchor.web3.Keypair.generate();
export const me = anchor.web3.Keypair.generate();

export const connection = provider.connection;
const splToken = Token;
const BN = anchor.BN;


describe("metaverse nft staking", async () => {

    anchor.setProvider(anchor.Provider.local());
    const program = anchor.workspace.MetaverseStaking as Program<MetaverseStaking>;
    const provider = program.provider;
    //console.log(program);

//const me = provider.wallet.payer;
    const authority = anchor.web3.Keypair.generate();
    const connection = provider.connection;

  it("working ", async () => {
  });

  it("request airdrop", async () => {
    await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(me.publicKey, 1000000000000),"confirmed");
    // Fund Main Account
    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: me.publicKey,
            toPubkey: me.publicKey,
            lamports: 10000000000,
          }),
        );
        return tx;
      })(),
      [me]
    );
  });


  it("working example", async () => {
    let mint1 = await splToken
        .createMint(
              connection, me,
              me.publicKey, null, 0,
              TOKEN_PROGRAM_ID);

    let wallet1 = await mint1.getOrCreateAssociatedAccountInfo(me.publicKey);
    let pool = anchor.web3.Keypair.generate();

    console.log(pool.publicKey.toBase58())
    await program.rpc.initializeStaking(
        new BN(10),
        new BN(10),
    {
      accounts: {
        rewardTokenMint: mint1.publicKey,
        authority: me.publicKey,
        stakingInstance: pool.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        allowedCollectionAddress: authority.publicKey,

        time: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,

      },
      signers: [me,pool],
      //instructions: [
      //    await program.account.stakingInstance.createInstruction(pool),
      //],
    });
    let result = await program.account.stakingInstance.fetch(pool.publicKey);
    console.log(result)
  });
});

