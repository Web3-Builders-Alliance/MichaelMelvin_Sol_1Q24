import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";

import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { readFile } from "fs/promises";
import wallet from "./wallet/wba-wallet.json";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    const imageBuffer = await readFile("./image/generug.png");
    const image = createGenericFile(imageBuffer, "generug.png");
    const [myUri] = await bundlrUploader.upload([image]);
    console.log("Your image URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
