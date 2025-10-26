mod log;
mod sigint;

use colored::Colorize as _;
use std::env;
use std::path::PathBuf;
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

    tracing::info!("Hello, world!");

    Ok(())
}
