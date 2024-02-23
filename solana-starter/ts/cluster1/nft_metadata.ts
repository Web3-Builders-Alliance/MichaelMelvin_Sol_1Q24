import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";

import { createBundlrUploader } from "@metaplex-foundation/umi-uploader-bundlr";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "./wallet/wba-wallet.json";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");
const bundlrUploader = createBundlrUploader(umi);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure
    // const image = ???
    // const metadata = {
    //     name: "?",
    //     symbol: "?",
    //     description: "?",
    //     image: "?",
    //     attributes: [
    //         {trait_type: '?', value: '?'}
    //     ],
    //     properties: {
    //         files: [
    //             {
    //                 type: "image/png",
    //                 uri: "?"
    //             },
    //         ]
    //     },
    //     creators: []
    // };
    // const myUri = ???
    // console.log("Your image URI: ", myUri);
    const image =
      "https://arweave.net/9bbQj3yrdW8LFALT8UkQ8KHzIRyFbdu6D-GRlny7piQ";
    const metadata = {
      name: "Generug",
      symbol: "GRUG",
      description: "Generug is a generative art project",
      image: image,
      attributes: [
        { trait_type: "Rarity", value: "Legendary" },
        { trait_type: "Generation", value: "1" },
      ],
      properties: {
        files: [
          {
            type: "image/png",
            uri: image,
          },
        ],
      },
      creators: [{ address: keypair.publicKey.toString(), share: 420 }],
    };

    const myUri = await bundlrUploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);
  } catch (error) {
    console.log("Oops.. Something went wrong", error);
  }
})();
