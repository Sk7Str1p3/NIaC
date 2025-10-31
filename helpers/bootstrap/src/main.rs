mod decrypt;
mod log;
mod sigint;

use anyhow::{Result, anyhow};
use colored::Colorize as _;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    log::init();
    sigint::init();

    let (flake, output) = {
        use std::env;
        use std::path::PathBuf;
        use tempdir::TempDir;

        let span = tracing::info_span!("dirs_setup");
        let _guard = span.enter();

        tracing::info!("Searching {flake}...", flake = "flake".blue());
        let flake = match &env::var("NIaC_SELF") {
            Ok(flake) => PathBuf::from(flake),
            Err(err) => {
                tracing::warn!(
                    "Failed to read {NIaC_SELF} environment variable: {why}
                                                                       Trying to use {PWD} instead...",
                    NIaC_SELF = "\"NIaC_SELF\"".blue().underline(),
                    why = err.to_string().yellow(),
                    PWD = "$PWD".yellow().underline()
                );
                match env::current_dir() {
                    Ok(pwd) => {
                        let mut pwd = pwd;
                        if !pwd.join("flake.nix").exists() {
                            tracing::warn!(
                                "Path '{pwd}' does not contain a {flake}, searching up...",
                                pwd = pwd.display().to_string().yellow(),
                                flake = "\"flake.nix\"".blue()
                            );
                        }
                        while !pwd.join("flake.nix").exists() {
                            if !pwd.pop() {
                                tracing::error!(
                                    "{FATAL} Failed to find flake root!",
                                    FATAL = "FATAL: ".red().bold()
                                );
                                return Err(anyhow!(
                                    "{FATAL} Failed to find flake root!",
                                    FATAL = "FATAL: ".red().bold()
                                ));
                            }
                        }
                        pwd
                    }
                    Err(err) => {
                        tracing::error!(
                            "{FATAL} Failed to find flake by $PWD: {why}",
                            FATAL = "FATAL:".bold().red(),
                            why = err.to_string().red().bold()
                        );
                        return Err(anyhow!(
                            "{FATAL} Failed to find flake: {why}",
                            FATAL = "FATAL: ".red().bold(),
                            why = err.to_string().red().bold()
                        ));
                    }
                }
            }
        };
        tracing::info!(
            "{Flake} {path}",
            Flake = "Flake:".blue().bold(),
            path = flake.display()
        );
        let flake = flake.join("secrets");

        let output = match TempDir::new("secrets") {
            Ok(tmp) => {
                let mut tmpdir = sigint::TMPDIR.lock().unwrap();
                *tmpdir = tmp.path().to_str().unwrap().into();
                tmp
            }
            Err(err) => {
                tracing::error!(
                    "{FATAL} Failed to create temporary directory: {why}",
                    FATAL = "FATAL:".red().bold(),
                    why = err.to_string().red().bold()
                );
                return Err(anyhow!(
                    "{FATAL} Failed to create temporary directory: {why}",
                    FATAL = "FATAL:".red().bold(),
                    why = err.to_string().red().bold()
                ));
            }
        };
        tracing::info!(
            "{OUT} {path}",
            OUT = "OUT:".blue().bold(),
            path = output.path().display()
        );

        (flake, output)
    };

    let (host, users) = {
        let span = tracing::info_span!("input");
        let _guard = span.enter();

        let host = loop {
            let input = dialoguer::Input::<'_, String>::new()
                .with_prompt("Host".blue().bold().underline().to_string())
                .interact_text()
                .map_err(|err| {
                    sleep(Duration::from_millis(1)); // Sleep to let SIGINT handler terminate program in case of Ctrl+C
                    err
                })?;

            if input.is_empty() {
                tracing::error!("{empty}", empty = "No hostname entered!".red());
                continue;
            }
            tracing::info!("Checking if host configuration exists...");
            let dir = flake.join("hosts").join(&input);
            if dir.exists() {
                break input;
            } else {
                tracing::error!(
                    "Folder {path} {missing}",
                    path = dir.to_string_lossy().underline(),
                    missing = "not found!".red()
                );
                println!(
                    "Hostname {name} is {invalid} Try again.",
                    name = input.red().underline(),
                    invalid = "invalid!".red().bold()
                );
            }
        };

        let users = loop {
            let input = dialoguer::Input::<'_, String>::new()
                .with_prompt("Users".blue().bold().underline().to_string())
                .interact_text()
                .map_err(|err| {
                    sleep(Duration::from_millis(1));
                    err
                })?
                .split(' ')
                .map(|s| s.into())
                .collect::<Vec<String>>();

            if input.is_empty() {
                tracing::error!("{empty}", empty = "No usernames entered".red());
                continue;
            }
            tracing::info!("Checking if all users configurations exist...");

            let mut invalid_users = Vec::<String>::new();
            for user in &input {
                let dir = flake.join("users").join(&user);
                if dir.exists() {
                    continue;
                } else {
                    tracing::error!(
                        "Folder {path} {missing}",
                        path = dir.to_string_lossy().underline(),
                        missing = "not found!".red()
                    );
                    invalid_users.push(user.into());
                }
            }

            if invalid_users.is_empty() {
                break input;
            } else {
                println!(
                    "Users {names} are {invalid} Try again.",
                    names = invalid_users
                        .iter()
                        .map(|user| user.red().underline().to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    invalid = "invalid".red().bold()
                )
            }
        };

        (host, users)
    };

    {
        use decrypt::sequoia::Decryptor;

        let span = tracing::info_span!("gpg_decrypt");
        let _guard = span.enter();
        tracing::info!("Decrypting master keys...");

        {
            let span = tracing::info_span!("host");
            let _guard = span.enter();
            tracing::info!("Decrypting master key for host {host}...");
            Decryptor::new()
                .input(flake.join("hosts").join(host).join("masterKey.gpg"))
                .output(output.path().join("host.masterKey.txt"))
                .build()?;
            tracing::info!("Successfully decrypted host key");
        }

        for user in users {
            tracing::info!("");
        }
    }

    tracing::info!("Hello, world!");

    Ok(())
}
