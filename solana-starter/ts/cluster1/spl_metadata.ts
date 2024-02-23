import {
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
  createMetadataAccountV3,
} from "@metaplex-foundation/mpl-token-metadata";
import {
  createSignerFromKeypair,
  publicKey,
  signerIdentity,
} from "@metaplex-foundation/umi";

import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "./wallet/wba-wallet.json";

// Define our Mint address
const mint = publicKey("Fm2rT9A1UG5RKkwCjuF7xEZnkLuTWptLPD6HZkQG7TJ9");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    const accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
    };
    const data: DataV2Args = {
      name: "mmelvin0x",
      symbol: "MM0X",
      uri: "",
      sellerFeeBasisPoints: 0,
      creators: null,
      collection: null,
      uses: null,
    };
    const args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: false,
      collectionDetails: null,
    };
    const tx = createMetadataAccountV3(umi, { ...accounts, ...args });
    const result = await tx.sendAndConfirm(umi);
    const signature = bs58.encode(result.signature);
    console.log(
      `Your metadata account was created: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
