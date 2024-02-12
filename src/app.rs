use crate::config::{Config, JimCryptAction};
use crate::{encrypt, decrypt};

pub fn run(config: Config) {
    match config.command {
        JimCryptAction::Encrypt => encrypt::encrypt(config.file),
        JimCryptAction::Decrypt { key } => decrypt::decrypt(config.file, key),
    };
}
