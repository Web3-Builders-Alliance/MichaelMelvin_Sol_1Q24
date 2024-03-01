import * as anchor from "@coral-xyz/anchor";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Account,
  TOKEN_PROGRAM_ID,
  createMint,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

import { Escrow } from "../target/types/escrow";
import { Program } from "@coral-xyz/anchor";
import { before } from "mocha";
import { randomBytes } from "crypto";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrow as Program<Escrow>;
  const maker = anchor.Wallet.local();
  const taker = Keypair.generate();
  const connection = program.provider.connection;
  const amountX = new anchor.BN(100 * LAMPORTS_PER_SOL);
  const amountY = new anchor.BN(100 * LAMPORTS_PER_SOL);

  let seed = new anchor.BN(randomBytes(8));
  let escrow: PublicKey;
  let mintX: PublicKey;
  let mintY: PublicKey;
  let makerAtaX: Account;
  let makerAtaY: Account;
  let takerAtaX: Account;
  let takerAtaY: Account;
  let escrowAtaX: PublicKey;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  before(async () => {
    await connection
      .requestAirdrop(maker.publicKey, LAMPORTS_PER_SOL * 10)
      .then(confirm)
      .then(log);
    await connection
      .requestAirdrop(taker.publicKey, LAMPORTS_PER_SOL * 10)
      .then(confirm)
      .then(log);

    mintX = await createMint(connection, maker.payer, maker.publicKey, null, 9);
    mintY = await createMint(connection, maker.payer, maker.publicKey, null, 9);
    makerAtaX = await getOrCreateAssociatedTokenAccount(
      connection,
      maker.payer,
      mintX,
      maker.publicKey
    );
    makerAtaY = await getOrCreateAssociatedTokenAccount(
      connection,
      maker.payer,
      mintY,
      maker.publicKey
    );
    takerAtaX = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintX,
      taker.publicKey
    );
    takerAtaY = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintX,
      taker.publicKey
    );
    escrow = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    escrowAtaX = await getAssociatedTokenAddress(mintX, escrow, true);
  });

  it("make", async () => {
    await program.methods
      .make(amountX, amountX, seed)
      .accounts({
        maker: maker.publicKey,
        escrow,
        mintX,
        mintY,
        escrowAtaX,
        makerAtaX: makerAtaX.address,
        makerAtaY: makerAtaY.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker.payer])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("take", async () => {
    await program.methods
      .take()
      .accounts({
        taker: taker.publicKey,
        maker: maker.publicKey,
        escrow,
        mintX,
        mintY,
        escrowAtaX,
        takerAtaX: takerAtaX.address,
        takerAtaY: takerAtaY.address,
        makerAtaY: makerAtaY.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([taker])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("refund", async () => {
    seed = new anchor.BN(randomBytes(8));
    escrow = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];
    escrowAtaX = await getAssociatedTokenAddress(mintX, escrow, true);

    await program.methods
      .make(amountX, amountX, seed)
      .accounts({
        maker: maker.publicKey,
        escrow,
        mintX,
        mintY,
        escrowAtaX,
        makerAtaX: makerAtaX.address,
        makerAtaY: makerAtaY.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker.payer])
      .rpc()
      .then(confirm)
      .then(log);

    await program.methods
      .refund()
      .accounts({
        maker: maker.publicKey,
        escrow,
        mintX,
        mintY,
        escrowAtaX,
        makerAtaX: makerAtaX.address,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([maker.payer])
      .rpc()
      .then(confirm)
      .then(log);
  });
});
