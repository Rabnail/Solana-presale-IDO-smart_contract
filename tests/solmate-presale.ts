import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolmatePresale } from "../target/types/solmate_presale";

describe("solmate-presale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolmatePresale as Program<SolmatePresale>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
