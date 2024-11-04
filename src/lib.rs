/// https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses

use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sha256(input: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

#[wasm_bindgen]
pub fn ripemd160(input: &[u8]) -> Vec<u8> {
    let mut hasher = Ripemd160::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

#[wasm_bindgen]
pub fn generate_bitcoin_address(public_key_hex: &str) -> String {
    // Convert hex to bytes
    let public_key_bytes = hex::decode(public_key_hex).expect("Invalid hex string");

    // Step 1: SHA-256 hashing
    let sha256_hash = sha256(&public_key_bytes);

    // Step 2: RIPEMD-160 hashing
    let ripemd160_hash = ripemd160(&sha256_hash);

    // Step 3: Add version byte (0x00 for Main Network)
    let mut versioned_hash = vec![0x00]; // Version byte
    versioned_hash.extend(ripemd160_hash);

    // Step 4: SHA-256 on the extended RIPEMD-160 result
    let checksum1 = sha256(&versioned_hash);
    let checksum2 = sha256(&checksum1);

    // Step 5: Take the first 4 bytes of the second SHA-256 hash (address checksum)
    let checksum = &checksum2[..4];

    // Step 6: Append the checksum to the versioned hash
    let mut final_address_bytes = versioned_hash;
    final_address_bytes.extend_from_slice(checksum);

    // Step 7: Convert the final byte array into a Base58 string
    // Use the `bs58` crate for Base58 encoding
    let bitcoin_address = bs58::encode(final_address_bytes).into_string();

    bitcoin_address
}
