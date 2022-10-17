use std::str::FromStr;

use bip32::DerivationPath;
use bip39::Mnemonic;
use cosmrs::crypto::secp256k1::SigningKey;

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

    let mut i = 1;
    let mut mnemonic = get_mnemonic();
    let cut = CHAIN.len() + 1;
    loop {
        if i % 1000 == 0 {
            println!("Tried {}", i);
        }

        let addr = calc_address(&mnemonic.to_seed(""), &path);
        if addr[cut..].starts_with(DESIRED) {
            println!("Got a match: {}", addr);
            println!("Mnemonic: {}", mnemonic);
            return
        }

        // update for next rounds
        mnemonic = get_mnemonic();
        i+=1;
    }
}
