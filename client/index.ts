import { Keypair, PublicKey, Connection } from '@solana/web3.js'
import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';

import { IDL } from "../target/types/solmate_presale";
import { getKeypairFromString, PROGRAM_ID, safeAirdrop } from './utils';
import { initialize } from './actions';
import { NATIVE_MINT } from '@solana/spl-token';
import { BN } from 'bn.js';
import { readFileSync } from 'fs';


async function main() {
  // Establish connection to the cluster
  // const connection = await establishConnection("http://127.0.0.1:8899");
  const connection = new Connection("https://api.devnet.solana.com");

  const payerKeystring = readFileSync("./keypairs/payer.key").toString();
  const payerWallet = getKeypairFromString(payerKeystring);

  const authorityKeystring = readFileSync("./keypairs/authority.key").toString();
  const authority = getKeypairFromString(authorityKeystring);

  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(payerWallet), anchor.AnchorProvider.defaultOptions());
  const program = new anchor.Program(IDL, PROGRAM_ID, provider);

  // Initialize or update
  const startDate = Math.ceil(Date.now() / 1000);
  const endDate = startDate + 60 * 60 * 24 * 30;

  const tx = await initialize(program, authority, new BN(startDate), new BN(endDate));
  console.log(tx);

}

main().then(
  () => process.exit(),
  (err) => {
    console.error(err);
    process.exit(-1);
  }
);
