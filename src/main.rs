pub mod config;
pub mod app;
pub mod encrypt;
pub mod decrypt;
//pub mod polynomial;
pub mod shared;

use crate::config::Config;

use clap::Parser;

fn main() {
    let config = Config::parse();
    app::run(config);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn polynomial_multiplication() {
        use jimcrypt::polynomial::Polynomial;

        let polynomial0: Polynomial = Polynomial::new(vec![5u8, 0, 10, 6].into());
        let polynomial1: Polynomial = Polynomial::new(vec![1u8, 2, 4].into());
        let polynomial2 = polynomial0 * polynomial1;
        assert_eq!(polynomial2, Polynomial::new(vec![5u8, 10, 30, 26, 52, 24].into()));
    }

    #[test]
    fn test_polynomial_string() {
        use jimcrypt::polynomial::Polynomial;

        let polynomial = Polynomial::random_polynomial(32);
        let same_poly = Polynomial::from_str(&polynomial.to_string()).expect("failed from_str");

        for i in 0..polynomial.len() {
            assert_eq!(polynomial[i], same_poly[i]);
        }

        assert_eq!(polynomial, same_poly);
    }

}

