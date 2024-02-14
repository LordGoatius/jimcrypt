use std::io::Write;
use std::{fs, path::Path, u8};

use jimcrypt::polynomial::Polynomial;
use crate::shared::*;

pub fn decrypt(file: Box<Path>, key: Polynomial) {
    let file = fs::read(file).expect("Failed to parse file");

    // We now have a vec of bytes as the file. Let's split it into a vec of [u8, 1024]
    let chunks: Vec<[u8; 32]> = file
        .into_iter()
        .collect::<Vec<u8>>()
        .chunks(32)
        .map(|chunk| vec_option_u8_to_array_32(TryInto::<Vec<u8>>::try_into(chunk).expect("failed into vec")))
        .collect();

    // We now have a fixed size number of arrays of 32 bytes (256 bits)
    
    
    let decrypted: Vec<[u8; 32]> = chunks
        .into_iter()
        .map(|chunk| xor_chunk(chunk, &key))
        .collect();



    let mut decrypted_file = fs::File::create("decrypted.jimdc").expect("could not create decrypted file");
    for chunk in decrypted.iter() {
        decrypted_file.write_all(chunk).expect("failed to write to encrypted file");
    }
}
