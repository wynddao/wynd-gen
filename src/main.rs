use std::str::FromStr;

use bip32::DerivationPath;
use bip39::Mnemonic;
use cosmrs::crypto::secp256k1::SigningKey;
use rand_core::{OsRng, RngCore};

const DESIRED: &str = "wy";
const CHAIN: &str = "juno";

fn get_seed() -> [u8;32] {
    let mut res = [0u8;32];
    OsRng.fill_bytes(&mut res);
    res
}

fn calc_address(seed: &[u8], path: &DerivationPath) -> String {
    let key = SigningKey::derive_from_path(seed, path).unwrap();
    let account = key.public_key().account_id(CHAIN).unwrap();
    account.as_ref().to_string()
}

fn main() {
    let path = DerivationPath::from_str("m/44'/118'/0'/0/0").unwrap();
    let expected_runs = 32u64.pow(DESIRED.len() as u32);
    println!("Estimating {} tries...", expected_runs);

    let mut i = 1;
    let mut seed = get_seed();
    let cut = CHAIN.len() + 1;
    loop {
        if i % 1000 == 0 {
            println!("Tried {}", i);
        }

        let addr = calc_address(&seed, &path);
        if addr[cut..].starts_with(DESIRED) {
            println!("Got a match: {}", addr);
            let phrase = Mnemonic::from_entropy(&seed).unwrap();
            println!("Mnemonic: {}", phrase);
            return
        }

        // update for next rounds
        seed = get_seed();
        i+=1;
    }
}
