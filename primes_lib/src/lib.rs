use is_prime::is_prime;
use num::bigint::{RandBigInt, ToBigInt, ToBigUint};
use num::Integer;
use num::{BigInt, BigUint, FromPrimitive};

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

pub fn gcd_extended(a: BigUint, b: BigUint) -> (BigUint, BigInt, BigInt) {
    if a == BigUint::from(0u8) {
        return (b, BigInt::from(0u8), BigInt::from(1u8));
    }

    let (gcd, x1, y1) = gcd_extended(b.clone() % a.clone(), a.clone());

    let x = y1 - (BigInt::from(b) / BigInt::from(a)) * x1.clone();
    let y = x1;

    (gcd, x, y)
}

#[allow(non_snake_case)]
pub fn mod_mult_inverse(totient_N: &BigUint, e: &BigUint) -> Result<BigUint, ()> {
    let (gcd, x, _) = gcd_extended(e.clone(), totient_N.clone());

    if gcd != BigUint::from(1u8) {
        Err(())
    } else {
        Ok(
            ((x % BigInt::from(totient_N.clone()) + BigInt::from(totient_N.clone()))
                % BigInt::from(totient_N.clone()))
            .to_biguint()
            .ok_or(())?,
        )
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{carmichaels_totient, find_e, mod_mult_inverse, rand_prime};
    use is_prime::is_prime as extern_is_prime;
    use num::BigUint;

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
        let d = mod_mult_inverse(&totient, &e).unwrap();
        assert_eq!((e * d) % totient, BigUint::from(1u8))
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
