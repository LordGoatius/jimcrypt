use std::io::Write;
use std::{fs, path::Path, u8};

use jimcrypt::polynomial::Polynomial;
use crate::shared::*;

pub fn encrypt(file: Box<Path>) {
    let file = fs::read(file).expect("Failed to parse file");

    // We now have a vec of bytes as the file. Let's split it into a vec of [u8, 32]
    let chunks: Vec<[u8; 32]> = file
        .into_iter()
        .collect::<Vec<u8>>()
        .chunks(32)
        .map(|chunk| vec_option_u8_to_array_32(TryInto::<Vec<u8>>::try_into(chunk).expect("failed into vec")))
        .collect();

    // We now have a fixed size number of arrays of 32 bytes (256 bits)
    let key: Polynomial = Polynomial::random_polynomial(32);

    let encrypted: Vec<[u8; 32]> = chunks
        .into_iter()
        .map(|chunk| xor_chunk(chunk, &key))
        .collect();
    let mut key_file = fs::File::create("key.jimkey").expect("could not create key file");
    key_file.write_all(key.to_string().as_bytes()).expect("failed to write to key file");

    let mut encrypted_file = fs::File::create("encrypted.jimc").expect("could not create encrypted file");
    for chunk in encrypted.iter() {
        encrypted_file.write_all(chunk).expect("failed to write to encrypted file");
    }
}
