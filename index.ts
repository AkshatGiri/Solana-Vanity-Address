import { Keypair } from "@solana/web3.js";
import bs58 from "bs58";

function generateVanityAddress(prefix: string) {
  const desiredPrefix = prefix;
  let found = false;
  let attempts = 0;

  while (!found) {
    const keypair = Keypair.generate();
    const publicKeyBase58 = keypair.publicKey.toBase58();

    if (publicKeyBase58.startsWith(desiredPrefix)) {
      found = true;
      console.log(
        `Found a keypair with prefix ${desiredPrefix} after ${attempts} attempts`
      );
      console.log(`Public Key: ${publicKeyBase58}`);
      console.log(`Secret Key: ${bs58.encode(keypair.secretKey)}`);
    }

    attempts++;
    if (attempts % 100000 === 0) {
      console.log(`Checked ${attempts} keys so far...`);
    }
  }
}

generateVanityAddress("four");
