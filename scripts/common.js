import {
    PublicKey, Transaction
} from '@solana/web3.js'
import pkg, { ASSOCIATED_TOKEN_PROGRAM_ID, Token } from "@solana/spl-token"
const { TOKEN_PROGRAM_ID } = pkg
import anchor from '@project-serum/anchor'

export const programID = new PublicKey("EXZJ3cCRD8dxmTYPxY4nWEWjGiwiW1U9yz2jzJzngnLg")
export const candyV2 = new PublicKey("DihbqTnmEfYYhWqrnuE9DZhok9uBdPj9WyyRqiS9JJVi")
export const rewAuth = new PublicKey("2caL4FyvxvLsZeQUnoEQd1FEgx9GXfmvguCpx2B2sbSA")
export const tokenMetadata = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
export const rewardToken = new PublicKey("A8DvXSStUYZhUdMZzammXDQvL7SKyFHQjLQTVZuXQsYh")
export const owner = new PublicKey("6iXei5r2jeshNeQ6tGgLawfDHkyHCsY58G4CFs86hqMj")
export const seeds = {
    user: anchor.utils.bytes.utf8.encode("user_deposit"),
    staking: "staking_instance",
    meta: "metadata",
    metadata: anchor.utils.bytes.utf8.encode("sidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
    token: anchor.utils.bytes.utf8.encode("sidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"),
}
export const opts = {
    preflightCommitment: "recent",
};

const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID = new PublicKey(
    'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL',
  );
  
export async function findAssociatedTokenAddress(
      walletAddress,
      tokenMintAddress
  ) {
      return (await PublicKey.findProgramAddress(
          [
              walletAddress.toBuffer(),
              TOKEN_PROGRAM_ID.toBuffer(),
              tokenMintAddress.toBuffer(),
          ],
          SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID
      ))[0];
  }


export async function getOrCreateAssociatedTokenAccount(
    connection,
    mint,
    wallet, 
    payer
) {
    const address = await findAssociatedTokenAddress(
        wallet,
        mint
        );
    if (!(await connection.getAccountInfo(address))) {
        const txn = new Transaction().add(Token.createAssociatedTokenAccountInstruction(
            ASSOCIATED_TOKEN_PROGRAM_ID,
            TOKEN_PROGRAM_ID,
            mint,
            address,
            wallet,
            payer.publicKey
        ))
        txn.recentBlockhash = (
            await connection.getRecentBlockhash()
        ).blockhash;
        txn.sign(payer)
        let signature = await connection.sendRawTransaction(txn.serialize());
        let confirmed = await connection.confirmTransaction(signature);
        console.log(signature);
    }
    return address;
}