//! ## ROPS
//! Module for decrypting sops-encrypted secrets

// TODO: add tests

use std::{fs, path::Path};

use rops::{
    cryptography::{cipher::AES256GCM, hasher::SHA512},
    file::{RopsFile, format::JsonFileFormat, state::EncryptedFile},
};

/// Decryptor for SOPS secret
pub struct Decryptor<In, Out>
where
    In: AsRef<Path> + Default,
    Out: AsRef<Path> + Default,
{
    key: String,
    enc_path: In,
    dec_path: Out,
}

impl<I, O> Decryptor<I, O>
where
    I: AsRef<Path> + Default,
    O: AsRef<Path> + Default,
{
    pub fn new() -> Self {
        Self {
            key: "".to_owned(),
            enc_path: I::default(),
            dec_path: O::default(),
        }
    }

    /// Define key to decrypt secret
    pub fn key(&mut self, key: String) -> &mut Self {
        self.key = key;
        self
    }

    /// Set path of encrypted key
    pub fn input(&mut self, path: I) -> &mut Self {
        self.enc_path = path;
        self
    }

    /// Set path where decrypted secret should be places
    pub fn output(&mut self, path: O) -> &mut Self {
        self.dec_path = path;
        self
    }

    /// Build decryptor and decrypt secret
    pub fn build(&self) -> Result<(), String> {
        let input = self.enc_path.as_ref();
        let output = self.dec_path.as_ref();

        let old_key = std::env::var("ROPS_AGE");
        unsafe { std::env::set_var("ROPS_AGE", &self.key) };

        let enc_str = std::fs::read_to_string(input).map_err(|e| e.to_string())?;
        let dec_str = enc_str
            .parse::<RopsFile<EncryptedFile<AES256GCM, SHA512>, JsonFileFormat>>()
            .map_err(|e| e.to_string())?
            .decrypt::<JsonFileFormat>()
            .map(|dec| dec.map().to_string())
            .map_err(|e| e.to_string())?;

        // TODO: remove this then ROPS implement binary format support
        let dec_json = serde_json::from_str::<'_, serde_json::Value>(&dec_str).unwrap();
        let dec_str = dec_json["data"].as_str().unwrap();
        fs::write(output, dec_str.as_bytes()).map_err(|e| e.to_string())?;

        unsafe {
            if let Ok(old) = old_key {
                std::env::set_var("ROPS_AGE", old);
            } else {
                std::env::remove_var("ROPS_AGE");
            }
        }

        Ok(())
    }
}
