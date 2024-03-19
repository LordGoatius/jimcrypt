use num::{BigUint, FromPrimitive, BigInt};
use num::bigint::RandBigInt;
use num::Integer;
use is_prime::is_prime;

pub fn rand_prime(size: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut p = rng.gen_biguint(size);

    if p.clone() % 2u8 == BigUint::from(0u8) { 
        p += 1u8; 
    }

    while !is_prime(&p.to_string()[..]) {
        p += 2u8;
    }

    p
}

#[inline]
pub fn carmichaels_totient(p: &BigUint, q: &BigUint) -> BigUint {
    (p - 1u8).lcm(&(q - 1u8))
}

pub fn find_e(totient: &BigUint) -> BigUint {
    let mut e = BigUint::from_u32(65537).unwrap();

    if totient % e.clone() == BigUint::from(0u8) {
        while e.gcd(&totient) != BigUint::from_u8(1u8).unwrap() {
            while !is_prime(&e.to_string()[..]) {
                e += 2u8;
            }
        }
    }

    e % totient.clone()
}

pub fn gcd_extended(a: BigUint, b: BigUint, x: &mut BigInt, y: &mut BigInt) -> BigUint {
    if a == BigUint::from(0u8) {
        *x = BigInt::from(0u8);
        *y = BigInt::from(1u8);
        return b;
    }

    let mut x1: BigInt = BigInt::from(0u8);
    let mut y1: BigInt = BigInt::from(0u8);

    let gcd = gcd_extended(b.clone() % a.clone(), a.clone(), &mut x1, &mut y1);

    *x = y1 - (BigInt::from(b) / BigInt::from(a)) * x1.clone();
    *y = x1;

    gcd
}

#[allow(non_snake_case)]
pub fn mod_mult_inverse(totient_N: &BigUint, e: &BigUint) -> Result<BigUint, ()> {
    let mut x: BigInt = BigInt::from(0u8);
    let mut y: BigInt = BigInt::from(0u8);
    let gcd = gcd_extended(e.clone(), totient_N.clone(), &mut x, &mut y);

    if gcd != BigUint::from(1u8) {
        Err(())
    } else {
        Ok(((x % BigInt::from(totient_N.clone()) + BigInt::from(totient_N.clone())) % BigInt::from(totient_N.clone())).to_biguint().ok_or(())?)
    }
}



#[cfg(test)]
pub mod tests {
    use is_prime::is_prime as extern_is_prime;
    use num::BigUint;
    use crate::{carmichaels_totient, find_e, mod_mult_inverse, rand_prime};

    #[test]
    fn is_prime() {
        let p = rand_prime(512);

        assert_eq!(extern_is_prime(&p.to_string()[..]), true);
    }

    #[test]
    fn semiprime() {
        let p = rand_prime(512);
        let q = rand_prime(512);

        assert_eq!(extern_is_prime(&(p * q).to_string()[..]), false);
    }

    #[test]
    fn test_modpow() {
        let p = rand_prime(28);
        let q = rand_prime(28);
        let m = rand_prime(13);

        let pq_mod_m = p.modpow(&q, &m); 

        eprintln!("{}", pq_mod_m);
    }

    #[test]
    fn mod_mult_inv() {
        let p = rand_prime(28);
        let q = rand_prime(28);

        let totient = carmichaels_totient(&p, &q);
        let e = find_e(&totient);
        let d = mod_mult_inverse(&totient, &e);
    }

    #[test]
    fn encrypt_decrypt() {
        let p = rand_prime(28);
        let q = rand_prime(28);
        #[allow(non_snake_case)]
        let N = p.clone() * q.clone();

        let totient = carmichaels_totient(&p, &q);
        let e = find_e(&totient);
        let d = mod_mult_inverse(&totient, &e).expect("Doesn't exist");

        let message = BigUint::from(645u16);
        let encrypted = message.clone().modpow(&e, &N);

        let decrypted = encrypted.modpow(&d, &N);

       assert_eq!(message, decrypted);
    }
}
