use std::path::Path;

use clap::{Parser, Subcommand};

use jimcrypt::polynomial::Polynomial;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// File to be decrypted or encrypted
    pub file: Box<Path>,

    #[command(subcommand)]
    pub command: JimCryptAction
}

#[derive(Subcommand, Debug)]
pub enum JimCryptAction {
    /// Encrypts file
    Encrypt,
    /// Decrypts file
    Decrypt {
        #[arg(short, long, required=true)]
        key: Polynomial,
    }
}
