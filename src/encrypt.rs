use std::{fs, path::Path, u8};

use crate::polynomial::Polynomial;

pub fn encrypt(file: Box<Path>, key: u64) {
    let file = fs::read(file).expect("Failed to parse file");
    // We now have a vec of bytes as the file. Let's split it into a vec of [u8, 1024]
    let chunks: Vec<[Option<u8>; 1024]> = file
        .into_iter()
        .map(Some)
        .collect::<Vec<Option<u8>>>()
        .chunks(1024)
        .map(|chunk| vec_option_u8_to_array(TryInto::<Vec<Option<u8>>>::try_into(chunk).expect("failed into vec")))
        .collect();
    // We now have a fixed size number of arrays of 1024 bytes
    let key: Polynomial = Polynomial::random_polynomial(128);
}

fn vec_option_u8_to_array(vec: Vec<Option<u8>>) -> [Option<u8>; 1024] {
    if vec.len() == 1024 {
        return TryInto::<[Option<u8>; 1024]>::try_into(vec).unwrap()
    } else {
        let mut ret: [Option<u8>; 1024] = [None; 1024];
        for (i, item) in vec.into_iter().enumerate() {
            ret[i] = item;
        }
        ret
    }
}
