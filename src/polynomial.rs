use std::ops::{Index, Mul};
use rand::{self, Rng};

/// Polynomial with integers mod 0x100 as group
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    coefficients: Vec<u8>
}

impl Polynomial {
    pub const fn new(coefficients: Vec<u8>) -> Self {
        Self {
            coefficients
        }
    }

    const fn is_prime(&self) -> bool {
        todo!()
    }

    fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn random_polynomial(length: usize) -> Polynomial {
        let mut rng = rand::thread_rng();
        let mut coefficients: Vec<u8> = vec![0u8; length];

        for i in 0..length {
            coefficients[i] = rng.gen_range(0u8..255u8);
        }

        Polynomial { coefficients }
    }
}

impl Index<usize> for Polynomial {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Self) -> Self::Output {
        let size_lhs = self.len();
        let size_rhs = rhs.len();

        let mut coefficients: Vec<u8> = vec![0; size_lhs + size_rhs - 1];

        for i in 0..size_lhs {
            for j in 0..size_rhs {
                coefficients[i + j ] = coefficients[i + j].wrapping_add(self[i].wrapping_mul(rhs[j]));
            }
        }

        Polynomial {
            coefficients
        }
    }
}
