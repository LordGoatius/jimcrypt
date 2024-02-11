use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// File to be decrypted or encrypted
    pub file: Box<Path>,

    #[command(subcommand)]
    pub command: JimCryptAction
}

#[derive(Subcommand)]
pub enum JimCryptAction {
    /// Encrypts file
    Encrypt {
        #[arg(short, long, required=false)]
        key: u64,
    },
    /// Decrypts file
    Decrypt {
        #[arg(short, long, required=true)]
        key: u64,
    }
}
