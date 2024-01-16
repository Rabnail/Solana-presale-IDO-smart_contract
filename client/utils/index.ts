import * as anchor from '@project-serum/anchor';
import { bs58 } from '@project-serum/anchor/dist/cjs/utils/bytes';
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey';
import { PublicKey, LAMPORTS_PER_SOL, Keypair } from '@solana/web3.js';

export const PREFIX = 'SOLMATE';
export const VAULT = 'VAULT';
export const SALE = 'SALE';

export const TOTAL_SUPPLY = new anchor.BN(1_000_000_000).mul(new anchor.BN(LAMPORTS_PER_SOL));

export const PROGRAM_ID = new PublicKey("HXbR9vT3guUKtip1Z88R3hxRNpEaScV97fwn8nYsL54D");

export const sleep = async (seconds) => {
  await new Promise(f => setTimeout(f, 1000 * seconds));
}

export function getKeypairFromString(str: string): Keypair {
  return Keypair.fromSecretKey(bs58.decode(str));
}

export async function safeAirdrop(connection: anchor.web3.Connection, key: anchor.web3.PublicKey, amount: number) {

  while (await connection.getBalance(key) < amount * LAMPORTS_PER_SOL) {
    try {
      await connection.confirmTransaction(
        await connection.requestAirdrop(key, LAMPORTS_PER_SOL),
        "confirmed"
      );
    } catch { }
  };
}

export const findGlobalAccount = (): PublicKey => {
  let [pubkey, bump] = findProgramAddressSync(
    [Buffer.from(PREFIX)],
    PROGRAM_ID,
  );

  return pubkey;
}
export const findSaleAccount = (
  authority: PublicKey
): PublicKey => {
  let [pubkey, bump] = findProgramAddressSync(
    [Buffer.from(PREFIX), authority.toBuffer(), Buffer.from(SALE)],
    PROGRAM_ID,
  );

  return pubkey;
}
export const findVaultAccount = (
  mint: PublicKey
): PublicKey => {
  let [pubkey, bump] = findProgramAddressSync(
    [Buffer.from(PREFIX), mint.toBuffer(), Buffer.from(VAULT)],
    PROGRAM_ID,
  );

  return pubkey;
}