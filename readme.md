[![Rust](https://github.com/RGGH/bitcoin_hashing_wasm/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/bitcoin_hashing_wasm/actions/workflows/rust.yml)
# ⚡ Base58 address from Public Key 🔑
---
### example key to enter: 

```0250863ad64a87ae8a2fe83c1af1a8403cb53f53e486d8511dad8a04887e5b2352```

This will give you :
```1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs```

[Visit the Bitcoin Wiki](https://en.bitcoin.it/wiki/Technical_background_of_version_1_Bitcoin_addresses)

This is a demo, it uses Legacy Bitcoin addresses, which start with a 1 for P2PKH (Pay-to-Public-Key-Hash) addresses.

    Legacy Addresses (Base58Check):
        Format: Starts with 1.
        Example: 1PMycacnJaSqwwJqjawXBErnLsZ7RkXUAs.
        Encoding: Base58Check encoding.
        Type: P2PKH (Pay-to-Public-Key-Hash).

This code generates a legacy Bitcoin address from a given public key in hexadecimal format. It performs the following steps:

1. **Hex to Bytes**: Converts the input hexadecimal string into a byte array.
2. **SHA-256 Hashing**: Computes the SHA-256 hash of the public key bytes.
3. **RIPEMD-160 Hashing**: Hashes the SHA-256 result using the RIPEMD-160 algorithm.
4. **Version Byte**: Prepends a version byte (0x00) to the RIPEMD-160 hash, indicating it's for the main Bitcoin network.
5. **Checksum Calculation**: Computes the SHA-256 hash of the versioned hash, then performs SHA-256 again to derive a checksum.
6. **Checksum Append**: Appends the first four bytes of the checksum to the versioned hash.
7. **Base58 Encoding**: Converts the final byte array into a Base58 string, resulting in the Bitcoin address.

   [try it out](https://rggh.github.io/bitcoin_hashing_wasm/web/)

   ![image](https://github.com/user-attachments/assets/560aed1d-fd47-4996-b579-e6f7b8a773c0)

