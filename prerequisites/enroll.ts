import { Address, AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  clusterApiUrl,
} from "@solana/web3.js";
import { IDL, WbaPrereq } from "./programs/wba_prereq";

import wallet from "./wba-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection(clusterApiUrl("devnet"));

const github = Buffer.from("mmelvin0x", "utf-8");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program = new Program<WbaPrereq>(
  IDL,
  "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1" as Address,
  provider,
);

const enrollmentSeeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollmentAccount] = PublicKey.findProgramAddressSync(
  enrollmentSeeds,
  program.programId,
);

(async () => {
  try {
    const txHash = await program.methods
      .complete(github)
      .accounts({
        signer: keypair.publicKey,
        prereq: enrollmentAccount,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${txHash}?cluster=devnet`,
    );
  } catch (error: any) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();
