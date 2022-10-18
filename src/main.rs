use std::str::FromStr;

use bip32::DerivationPath;
use bip39::Mnemonic;
use cosmrs::crypto::secp256k1::SigningKey;
use rayon::prelude::*;
const DESIRED: &str = "wynd";
const CHAIN: &str = "juno";
const WORD_COUNT: usize = 12;

fn get_mnemonic() -> Mnemonic {
    Mnemonic::generate(WORD_COUNT).unwrap()
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

    let cut = CHAIN.len() + 1;
    let start_time_ms = std::time::Instant::now();
    if let Some((addr, mnemonic)) = (1..).par_bridge().find_map_any(|i| {
        let mnemonic = get_mnemonic();
        if i % 2000 == 0 {
            let current_time_ms = std::time::Instant::now();
            let elapsed_ms = current_time_ms.duration_since(start_time_ms).as_millis();
            println!("Tried {} mnemonics in {} ms", i, elapsed_ms);
        }

        let addr = calc_address(&mnemonic.to_seed(""), &path);
        if addr[cut..].starts_with(DESIRED) {
            return Some((addr, mnemonic));
        }

        None
    }) {
        println!("Found a match: {}", addr);
        println!("Mnemonic: {}", mnemonic);
    } else {
        println!("No match found");
    }
}
