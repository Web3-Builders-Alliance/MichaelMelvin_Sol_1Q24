import { Keypair } from "@solana/web3.js";
import base58 from "bs58";
import fs from "fs";
import prompt from "prompt-sync";

const kp = Keypair.generate();
fs.writeFileSync("dev-wallet.json", `[${kp.secretKey.toString()}]`);

const promptUser = prompt();
console.log(
  `You've generated a new Solana wallet: ${kp.publicKey.toBase58()}
    To save your wallet, copy and paste the following into a JSON file:
    \n[${kp.secretKey.toString()}]
    \nWe have saved your wallet to a file called dev-wallet.json. Keep this file safe and secure. If you lose it, you could lose access to your wallet.`,
);

const uploadToPhantom = promptUser(
  "\nWould you like to upload this wallet to Phantom? (y/n)\n",
)
  .toLowerCase()
  .trim();

if (uploadToPhantom === "y") {
  console.log(
    `\nTo upload your wallet to Phantom, copy and paste the following into the Phantom wallet app:
      \n${walletToBase58(kp.secretKey)}`,
  );
}

export function walletToBase58(wallet: Uint8Array): string {
  return base58.encode(wallet);
}

export function base58ToWallet(base58String: string): Uint8Array {
  return base58.decode(base58String);
}
