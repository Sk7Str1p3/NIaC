//! ## GPG (Sequoia)
//! GPG decryption using `sequoia` crate - PGP implementation in pure rust
//!

use std::fs::File;
use std::io::Read;
use std::path::Path;

use colored::Colorize;
use dialoguer::Password;

use anyhow::Result;
use sequoia_openpgp::crypto::Password as PgpPasswd;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::parse::stream::{DecryptionHelper, DecryptorBuilder, VerificationHelper};
use sequoia_openpgp::policy::StandardPolicy;

#[derive(Clone)]
pub struct Decryptor<In, Out>
where
    In: AsRef<Path> + Default + Clone,
    Out: AsRef<Path> + Default + Clone,
{
    passwd: PgpPasswd,
    enc_path: In,
    dec_path: Out,
}

impl<In, Out> Decryptor<In, Out>
where
    In: AsRef<Path> + Default + Clone,
    Out: AsRef<Path> + Default + Clone,
{
    pub fn new() -> Self {
        Self {
            passwd: "".into(),
            enc_path: In::default(),
            dec_path: Out::default(),
        }
    }

    /// Set path of encrypted file
    pub fn input(&mut self, path: In) -> &mut Self {
        self.enc_path = path;
        self
    }

    /// Set path where decrypted secret should be placed
    pub fn output(&mut self, path: Out) -> &mut Self {
        self.dec_path = path;
        self
    }

    /// Build decryptor and decrypt file
    pub fn build(&mut self) -> Result<()> {
        let mut file = File::create(self.dec_path.as_ref())?;
        let mut attempts = 0;
        let policy = StandardPolicy::new();
        let message = {
            let mut buf = vec![];
            File::open(self.enc_path.as_ref())?.read(&mut buf)?;
            buf
        };
        while attempts < 3 {
            let passwd = Password::new().with_prompt(format!(
                "{} ({}: {}/3)",
                "Password".blue().underline(),
                "Attempt".red().bold(),
                attempts + 1
            ));
            self.passwd = passwd.interact()?.into();

            let mut decryptor = match DecryptorBuilder::from_bytes(&message)?.with_policy(
                &policy,
                None,
                self.clone(),
            ) {
                Result::Ok(dec) => dec,
                Err(_) => {
                    println!("w");
                    attempts += 1;
                    continue;
                }
            };

            std::io::copy(&mut decryptor, &mut file)?;
        }
        Result::Ok(())
    }
}

impl<In, Out> VerificationHelper for Decryptor<In, Out>
where
    In: AsRef<Path> + Default + Clone,
    Out: AsRef<Path> + Default + Clone,
{
    fn get_certs(
        &mut self,
        _ids: &[sequoia_openpgp::KeyHandle],
    ) -> sequoia_openpgp::Result<Vec<sequoia_openpgp::Cert>> {
        Ok(Vec::new())
    }

    fn check(
        &mut self,
        _structure: sequoia_openpgp::parse::stream::MessageStructure,
    ) -> sequoia_openpgp::Result<()> {
        Ok(())
    }
}

impl<In, Out> DecryptionHelper for Decryptor<In, Out>
where
    In: AsRef<Path> + Default + Clone,
    Out: AsRef<Path> + Default + Clone,
{
    fn decrypt(
        &mut self,
        _pkesks: &[sequoia_openpgp::packet::PKESK],
        skesks: &[sequoia_openpgp::packet::SKESK],
        _sym_algo: Option<sequoia_openpgp::types::SymmetricAlgorithm>,
        decrypt: &mut dyn FnMut(
            Option<sequoia_openpgp::types::SymmetricAlgorithm>,
            &sequoia_openpgp::crypto::SessionKey,
        ) -> bool,
    ) -> sequoia_openpgp::Result<Option<sequoia_openpgp::Cert>> {
        for skesk in skesks {
            if skesk
                .decrypt(&self.passwd)
                .map(|(algo, key)| decrypt(algo, &key))
                .unwrap_or(false)
            {
                return Ok(None);
            };
        }

        Err(anyhow::anyhow!("Wrong password"))
    }
}
