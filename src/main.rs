use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::Keypair;
use rayon::prelude::*;
use bs58;
use std::time::{Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::ThreadPoolBuilder;

fn main() {
    let desired_prefix = "akshat";  // Customize the prefix here

    let num_threads = rayon::current_num_threads();
    println!("Number of threads used by Rayon: {}", num_threads);

    rayon::iter::repeat(())
        .map(|_| {
            let pair = Keypair::new();
            pair
        })
        .find_any(|keypair| {
            let public_key_base58 = bs58::encode(keypair.pubkey()).into_string();
            if public_key_base58.starts_with(&desired_prefix) {
                println!("Found a keypair with prefix {}!", desired_prefix);
                println!("Public Key: {}", public_key_base58);
                // Handling the full private key correctly
                let seed = keypair.secret().as_ref(); // This gets the 32-byte seed
                let full_private_key = [seed, keypair.pubkey().as_ref()].concat(); // Concatenate seed and public key for the full 64 bytes

                println!("Secret Key (64 bytes): {}", bs58::encode(full_private_key).into_string());
                true
            } else {
                false
            }
        });
}

