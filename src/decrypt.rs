use std::{fs, path::Path};

pub fn decrypt(file: Box<Path>, key: u64) {
    let file = fs::read(file).expect("Failed to parse file");
}
