use std::ops::{Index, Mul};
use std::collections::VecDeque;
use std::str::FromStr;
use rand::{self, Rng};

/// Polynomial with integers mod 0x100 as group
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    coefficients: VecDeque<u8>
}

impl Polynomial {
    pub const fn new(coefficients: VecDeque<u8>) -> Self {
        Self {
            coefficients
        }
    }

    #[allow(dead_code)]
    const fn is_prime(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.coefficients.len()
    }

    pub fn random_polynomial(length: usize) -> Polynomial {
        let mut rng = rand::thread_rng();
        let mut coefficients: VecDeque<u8> = vec![0u8; length].into();

        for i in 0..length {
            coefficients[i] = rng.gen_range(0u8..=255u8);
        }

        Polynomial { coefficients }
    }
}

impl ToString for Polynomial {
    fn to_string(&self) -> String {
        let mut ret = String::new();
        for i in 0..self.len() {
            ret = format!("{}{:0>2x}", ret, self[i]);
        }
        return ret;
    }
}

impl Iterator for Polynomial {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        return self.coefficients.pop_front()
    }
}

/// let polynomial = Polynomial::new(vec![1, 2, 3, 4].into());
/// assert_eq(polynomial[0], 1); 
/// assert_eq(polynomial[1], 2); 
/// assert_eq(polynomial[2], 3); 
/// assert_eq(polynomial[3], 4); 
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

        let mut coefficients: VecDeque<u8> = vec![0; size_lhs + size_rhs - 1].into();

        for i in 0..size_lhs {
            for j in 0..size_rhs {
                coefficients[i + j] = coefficients[i + j].wrapping_add(self[i].wrapping_mul(rhs[j]));
            }
        }

        Polynomial {
            coefficients
        }
    }
}

impl FromStr for Polynomial {
    type Err = <u64 as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: VecDeque<u8> = s.chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|x| format!("{}{}", x[0], x[1]))
            .map(|str| u8::from_str_radix(&str, 16).expect("failed parse"))
            .collect();
        return Ok(Polynomial::new(bytes));
    }
}
