import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = new PublicKey("5q3JmFVTcvn2GHo5zurZbTs1p8c2zsivFLeZAHz78ppb");
const connection = new Connection(clusterApiUrl("devnet"));

(async () => {
  try {
    const balance = await connection.getBalance(from.publicKey);
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
      }),
    );

    const fee =
      (await connection.getFeeForMessage(transaction.compileMessage())).value ||
      0;
    transaction.instructions.pop();
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      }),
    );

    const recentBlockhash = (await connection.getRecentBlockhash("confirmed"))
      .blockhash;
    transaction.recentBlockhash = recentBlockhash;
    transaction.feePayer = from.publicKey;

    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);
    console.log(
      `Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`,
    );
  } catch (error) {
    console.error(`Oops, something went wrong: ${error}`);
  }
})();
