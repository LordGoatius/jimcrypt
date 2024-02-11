pub mod config;
pub mod app;
pub mod encrypt;
pub mod decrypt;
pub mod polynomial;

use crate::config::Config;

use clap::Parser;
use polynomial::Polynomial;

fn main() {
    let config = Config::parse();
    app::run(config);
}

#[cfg(test)]
mod tests {
    #[test]
    fn polynomial_multiplication() {
        use crate::polynomial::Polynomial;

        let polynomial0: Polynomial = Polynomial::new(vec![5u8, 0, 10, 6]);
        let polynomial1: Polynomial = Polynomial::new(vec![1u8, 2, 4]);
        let polynomial2 = polynomial0 * polynomial1;
        assert_eq!(polynomial2, Polynomial::new(vec![5u8, 10, 30, 26, 52, 24]));
    }
}

