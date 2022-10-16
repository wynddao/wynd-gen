use std::str::FromStr;
use bip32::DerivationPath;
use cosmrs::crypto::secp256k1::SigningKey;

// get seed bytes here
use rand_core::OsRng;

const desired: &str = "a";
const chain: &str = "juno";

fn calc_prefix(seed: &[u8], path: &DerivationPath) -> String {
    let key = SigningKey::derive_from_path(seed, path).unwrap();
    let account = key.public_key().account_id(chain).unwrap();
    let cut = chain.len() + 1;
    account.as_ref()[cut..].to_string()
}

fn main() {
    let path = DerivationPath::from_str("m/44'/118'/0'/0/0").unwrap();

    // TODO: generate from OsRng
    let seed = b"foobar12345678901234567890foobar"; // 32 bytes
    loop {
        let addr = calc_address(&seed, &path);
        if addr.starts_with(desired) {
            println!("Got a match: {}", addr);
            return
        }
        // TODO: update seed
    }
}
