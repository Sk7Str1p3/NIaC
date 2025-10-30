mod decrypt;
mod log;
mod sigint;

use colored::Colorize as _;
use std::env;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;
use tempdir::TempDir;

fn main() -> Result<(), String> {
    log::init();
    sigint::init();

    let (flake, output) = {
        let span = tracing::info_span!("dirs_setup");
        let _guard = span.enter();

        tracing::info!("Searching {}...", "flake".blue());
        let flake = match &env::var("NIaC_SELF") {
            Ok(flake) => PathBuf::from(flake),
            Err(err) => {
                tracing::warn!(
                    "Failed to read \"NIaC_SELF\" environment variable: {}
                                                                       Trying to use $PWD instead...",
                    err.to_string().yellow()
                );
                match env::current_dir() {
                    Ok(pwd) => {
                        let mut pwd = pwd;
                        if !pwd.join("flake.nix").exists() {
                            tracing::warn!(
                                "Path '{}' does not contain a {}, searching up...",
                                pwd.display().to_string().yellow(),
                                "\"flake.nix\"".blue()
                            );
                        }
                        while !pwd.join("flake.nix").exists() {
                            if !pwd.pop() {
                                return Err(format!(
                                    "{} Failed to find flake root!",
                                    "FATAL".red().bold()
                                ));
                            }
                        }
                        pwd
                    }
                    Err(err) => {
                        tracing::error!(
                            "{} Failed to find flake by $PWD: {}",
                            "FATAL:".bold().red(),
                            err.to_string().red().bold()
                        );
                        return Err(format!("Failed to find flake: {err}"));
                    }
                }
            }
        };
        tracing::info!("{} {}", "Flake:".blue().bold(), flake.display());
        let flake = flake.join("secrets");

        let output = match TempDir::new("secrets") {
            Ok(tmp) => {
                let mut tmpdir = sigint::TMPDIR.lock().unwrap();
                *tmpdir = tmp.path().to_str().unwrap().into();
                tmp
            }
            Err(err) => {
                tracing::error!(
                    "{} Failed to create temporary directory: {}",
                    "FATAL:".red().bold(),
                    err.to_string().red().bold()
                );
                return Err(format!(
                    "{} Failed to create temporary directory: {}",
                    "FATAL:".red().bold(),
                    err.to_string().red().bold()
                ));
            }
        };
        tracing::info!("{} {}", "OUT:".blue().bold(), output.path().display());

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
                    let 
                    err.to_string()
                })?;

            if input.is_empty() {
                tracing::error!("No hostname entered");
                continue;
            }
            tracing::info!("Checking if host configuration exists...");
            let dir = flake.join("hosts").join(&input);
            if dir.exists() {
                break input;
            } else {
                tracing::error!(
                    "Folder {} {}",
                    dir.to_string_lossy().underline(),
                    "not found!".red().bold()
                );
                println!(
                    "Hostname {} is {} Try again.",
                    input.red().underline(),
                    "invalid!".red().bold()
                );
            }
        };

        let users = loop {
            let input = dialoguer::Input::<'_, String>::new()
                .with_prompt("Users".blue().bold().underline().to_string())
                .interact_text()
                .map_err(|err| err.to_string())?
                .split(' ')
                .map(|s| s.into())
                .collect::<Vec<String>>();

            if input.is_empty() {
                tracing::error!("No usernames entered");
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
                        "Folder {} {}",
                        dir.display().to_string().underline(),
                        "not found!".red()
                    );
                    invalid_users.push(user.into());
                }
            }

            if invalid_users.is_empty() {
                break input;
            } else {
                println!(
                    "Users {} are {} Try again.",
                    invalid_users
                        .iter()
                        .map(|user| user.red().underline().to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    "invalid".red().bold()
                )
            }
        };

        (host, users)
    };

    {
        let span = tracing::info_span!("gpg_decrypt");
        let _guard = span.enter();
        tracing::info!("Decrypting master keys...");

        tracing::info!("Decrypting master key for host {host}");
        //decrypt::sequoia::decrypt(&flake.join(format!("hosts/{host}/masterKey.gpg")));
        tracing::info!("Successfully decrypted host key");

        for user in users {
            tracing::info!("");
        }
    }

    tracing::info!("Hello, world!");

    Ok(())
}
