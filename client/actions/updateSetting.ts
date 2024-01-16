import * as anchor from '@project-serum/anchor';
import { SYSVAR_RENT_PUBKEY, Keypair, PublicKey } from '@solana/web3.js';
import { Program } from '@project-serum/anchor';

import { SolmatePresale } from "../../target/types/solmate_presale";
import { findGlobalAccount } from '../utils';

export async function updateSetting(
    program: Program<SolmatePresale>,
    authority: Keypair,
    newAuthority: PublicKey,
    startDate: anchor.BN | null,
    endDate: anchor.BN | null,
    activated: boolean | null,
) {
    const globalState = findGlobalAccount();

    try {
        const tx = await program.methods.updateSetting(
            startDate,
            endDate,
            activated
        )
            .accounts({
                authority: authority.publicKey,
                newAuthority,
                globalState,
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
