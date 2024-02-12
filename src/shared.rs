use crate::polynomial::Polynomial;

pub fn xor_chunk(chunk: [u8; 32], polynomial: &Polynomial) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, byte) in chunk.iter().enumerate() {
        out[i] = byte^polynomial[i];
    }
    return out;
}

pub fn vec_option_u8_to_array_32(vec: Vec<u8>) -> [u8; 32] {
    if vec.len() == 32 {
        return TryInto::<[u8; 32]>::try_into(vec).unwrap()
    } else {
        let mut ret: [u8; 32] = [0; 32];
        for (i, item) in vec.into_iter().enumerate() {
            ret[i] = item;
        }
        ret
    }
}
