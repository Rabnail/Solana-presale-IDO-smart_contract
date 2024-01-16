import * as anchor from '@project-serum/anchor';
import { SYSVAR_RENT_PUBKEY, Keypair, PublicKey } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

import { SolmatePresale } from "../../target/types/solmate_presale";
import { findGlobalAccount } from '../utils';

export async function initialize(
    program: Program<SolmatePresale>,
    authority: Keypair,
    startDate: anchor.BN,
    endDate: anchor.BN
) {
    const globalAccount = findGlobalAccount();

    try {
        const tx = await program.methods.initialize(
            startDate,
            endDate
        )
            .accounts({
                authority: authority.publicKey,
                globalState: globalAccount,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([authority])
            .rpc();
        return tx;
    }
    catch (ex) {
        console.log(ex);
    }

};
