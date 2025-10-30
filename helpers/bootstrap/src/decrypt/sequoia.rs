//! ## GPG (Sequoia)
//! GPG decryption using `sequoia` crate - PGP implementation in pure rust
//!

use std::fs;

use dialoguer::Password;
use sequoia_openpgp::parse::{stream::DecryptorBuilder, Parse};

pub fn decrypt<P: AsRef<std::path::Path>>(path: P) -> Result<(), String> {
    tracing::info!("Decrypting key at {}", path.as_ref().display());

    // TODO: patch dialoguer to use '*' mask or use another crate
    let passwd = Password::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Password")
        .interact()
        .unwrap();

    let enc_data = fs::read(path).map_err(|err| err.to_string())?;
    let mut decryptor = DecryptorBuilder::from_bytes(&enc_data).unwrap().password();

    let decrypted = msg.decrypt_with_password()?;
    Ok(())
}
