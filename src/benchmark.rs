// To test how long it takes to generate 1 million keypairs 
// on your machine on a single thread. 
// This is a simple benchmark that can give us a rough idea
// how many keypairs we can generate even on multithreaded mode.
use solana_sdk::signature::{Keypair, Signer};
use bs58;
use std::time::{Instant, Duration};

fn main() {
    let num_pairs = 1_000_000;  // Number of key pairs to generate for the test
    let start_time = Instant::now();

    for _ in 0..num_pairs {
        let pair = Keypair::new();
        // Optionally, convert to base58 to simulate full processing
        let public_key_base58 = bs58::encode(pair.pubkey()).into_string();
    }

    let duration = start_time.elapsed();
    let duration_in_seconds = duration.as_secs_f64(); // Duration in seconds as a floating-point number
    println!("Generated {} key pairs in: {:.2} seconds", num_pairs, duration_in_seconds);
    println!("Rate: {:.2} pairs/sec", num_pairs as f64 / duration_in_seconds);
}