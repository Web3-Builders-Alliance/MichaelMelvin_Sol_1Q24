import {
  createNft,
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  generateSigner,
  percentAmount,
  signerIdentity,
} from "@metaplex-foundation/umi";

import base58 from "bs58";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "./wallet/wba-wallet.json";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
  // let tx = ???
  // let result = await tx.sendAndConfirm(umi);
  // const signature = base58.encode(result.signature);

  // console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

  const tx = createNft(umi, {
    mint,
    name: "Generug",
    symbol: "GRUG",
    uri: "https://arweave.net/6Sq0TL0rs8hAKvTQFjK5vOyMNEStPvRu0bxAW-u8EdQ",
    sellerFeeBasisPoints: percentAmount(4.2),
  });
  const result = await tx.sendAndConfirm(umi);
  const signature = base58.encode(result.signature);
  console.log(
    `Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );
  console.log("Mint Address: ", mint.publicKey);
})();
