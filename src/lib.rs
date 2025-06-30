use hex::{decode, encode};

pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, String> {
    // TODO: Decode hex string into Vec<u8>, return error string on failure

    if hex_str.len() % 2 != 0 {
        return Err("Decoded hex string must have an even length".to_string());
    }
    // I Check if the hex string has even length, Since the decoder interprets the bytes in pairs and Each byte requires exactly 2 hex characters (e.g., "ab" = 1 byte)

    decode(hex_str).map_err(|error| format!("Invalid hex: {}", error))
    // I Attempt to decode the input hex string into a vector of bytes. and uses .map_err to convert the error into a readable string format if decoding fails.
}

pub fn to_big_endian(bytes: &[u8]) -> Vec<u8> {
    // TODO: Reverse the byte order of input slice and return as Vec<u8>

    let mut result = Vec::from(bytes);
    result.reverse();
    result
    // I Creates a new Vec by copying data from slice then Reverses the Vec in-place and Return the modified Vec
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    // TODO: Implement conversion of bytes slice to hex string

    encode(bytes)
    // I Uses the hex crate's encode function to convert the byte slice into a hex string
}

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // TODO: Implement conversion of hex string to bytes vector

    decode(hex)
    // I Uses the hex crate's decode function to convert the hex string into a byte vector
}

pub fn swap_endian_u32(num: u32) -> [u8; 4] {
    // TODO: Implement little-endian byte swap for u32

    num.to_le_bytes()
    // I Converts the u32 number to its little-endian byte representation using the in_built to_le_bytes method, which returns an array of 4 bytes in little-endian order.
    // Big-endian: most significant byte first → 12 34 56 78
    // Little-endian: least significant byte first → 78 56 34 12
}

pub fn parse_satoshis(input: &str) -> Result<u64, String> {
    // TODO: Parse input string to u64, return error string if invalid
    input.parse::<u64>()
        .map_err(|_| "Invalid satoshi amount".to_string())
    // I Attempts to parse the input string as a u64 and returns an error string if parsing fails.
    // If the input is valid, it returns the parsed u64 value.
}

// pub enum ScriptType {
//     P2PKH,
//     P2WPKH,
//     Unknown,
// }

// pub fn classify_script(script: &[u8]) -> ScriptType {
//     // TODO: Match script pattern and return corresponding ScriptType
// }

// // TODO: complete Outpoint tuple struct
// pub struct Outpoint();

// pub fn read_pushdata(script: &[u8]) -> &[u8] {
//     // TODO: Return the pushdata portion of the script slice (assumes pushdata starts at index 2)
// }

// pub trait Wallet {
//     fn balance(&self) -> u64;
// }

// pub struct TestWallet {
//     pub confirmed: u64,
// }

// impl Wallet for TestWallet {
//     fn balance(&self) -> u64 {
//         // TODO: Return the wallet's confirmed balance
//     }
// }

// pub fn apply_fee(balance: &mut u64, fee: u64) {
//     // TODO: Subtract fee from mutable balance reference
// }

// pub fn move_txid(txid: String) -> String {
//     // TODO: Return formatted string including the txid for display or logging
// }

// // TODO: Add necessary derive traits
// pub enum Opcode {
//     OpChecksig,
//     OpDup,
//     OpInvalid,
// }

// impl Opcode {
//     pub fn from_byte(byte: u8) -> Result<Self, String> {
//         // TODO: Implement mapping from byte to Opcode variant
//     }
// }

// // TODO: Add necessary derive traits
// pub struct UTXO {
//     pub txid: Vec<u8>,
//     pub vout: u32,
//     pub value: u64,
// }

// pub fn consume_utxo(utxo: UTXO) -> UTXO {
//     // TODO: Implement UTXO consumption logic (if any)
// }
