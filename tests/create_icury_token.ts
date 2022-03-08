import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CreateIcuryToken } from "../target/types/create_icury_token";

describe("create_icury_token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.CreateIcuryToken as Program<CreateIcuryToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
