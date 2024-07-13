import { Keypair } from "@solana/web3.js";
import seed from "./keypair.json";

const seedIntArr = Uint8Array.from(seed);

const keypair = Keypair.fromSecretKey(seedIntArr);

// Get the private key (secret key)
const privateKey = Buffer.from(keypair.secretKey).toString("hex");
console.log("Private Key:", privateKey);

// Get the public key
const publicKey = keypair.publicKey.toBase58();
console.log("Public Key:", publicKey);
