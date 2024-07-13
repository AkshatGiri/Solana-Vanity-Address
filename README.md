# Solana Vanitiy Address Generator

This repo contains scripts to generate vanity addresses ( addresses that start with a specific prefix ) for solana.

For example, Iggy Azalea's wallet starts with "iggy" - iggyrnKkLxsT5JeErvc71G3SahppAoYG2tqMrGBk2gV

## Usage

There are 2 script in this repo. One writtein typescript and one in rust.

If you only want a 1-4 letter prefix you can use either the typescript or rust script.
However anything greater than that will take ALOT of generations hence alot of time. To do this faster use the rust script. It generates keypairs much much faster and the script runs mutlithreaded. Even with this optimized script a 6 letter prefix will take a long time to generate ( hours ). So you probably don't want to go beyond that. The scripts are case sensetive but that can be easily changed by lowercasing the prefix and the generated keypair before comparing them.

The number of attempts required to generate a vanity address is exponential to the number of characters in the prefix.

- 1 letter = 58 attempts
- 2 letters = 58^2 = 3364 attempts
- 3 letters = 58^3 = 195,112 attempts
- 4 letters = 58^4 = 11,316,496 attempts
- 5 letters = 58^5 = 656,356,768 attempts
- 6 letters = 58^6 = 38,068,692,544 attempts and it gets absurd from there.

The above list is a simple probability calculation and does not guarantee that we will find the answer in that many attempts.

NOTE: The prefix must only contain characters from the following list 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
Solana addresses do not include potentially ambiguous characters such as 0 (zero), O (uppercase o), I (uppercase i), and l (lowercase L) to avoid confusion. If you use characters outside of the above list, you will never get a match.

### Typescript script

1. `npm install`
2. Update the prefix in index.ts
3. `npm start`

### Rust Script

1. Update the prefix in `src/main.rs`
1. `cargo run --release`

## TODO

I would love to make a gpu accelerated version of keypair generation. If you have any ideas on how to do this please let me know.

## NOTE - July 12, 2024

1. I found that we can generate solana vanity addresses directly using the solana cli. It's faster than our rust script since it doesn't convert each generated address to base58 and skips over several addresses by eliminating invalid ones. Refer to this page for more info on this - https://solana.com/developers/cookbook/wallets/generate-vanity-address

2. I also found that someone has already made a gpu accelerated version of keypair generation. You can read more about it here - https://smith-mcf.medium.com/solana-vanity-address-using-gpus-5a68ad94d1d4
