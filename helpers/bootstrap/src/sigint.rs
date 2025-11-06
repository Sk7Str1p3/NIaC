//! ## SIGINT
//! This module provides functionality to gracefully handle
//! program interruption by cleaning up temporary
//! directories when the user presses Ctrl+C.
use std::sync::Mutex;

use color_eyre::Result;
use color_eyre::eyre::Context as _;

/// Holds the path to a temporary directory that needs to be
/// cleaned up when the program is interrupted.
///
/// The path stored in this variable is automatically
/// removed when a SIGINT signal (Ctrl+C) is received by the
/// program. This ensures proper cleanup of temporary
/// resources even during unexpected program termination.
///
/// ### Example
/// ```
/// use crate::sigint::TMPDIR;
/// let path = "/tmp/my_temp_dir";
/// // ...
/// tracing::info!("TMPDIR created at: {}", path)
/// // Store a temporary directory path in global state
/// *TMPDIR.lock().unwrap() = String::from(path);
/// ```
pub static TMPDIR: Mutex<String> = Mutex::new(String::new());

/// Initializes the SIGINT (Ctrl+C) handler to gracefully
/// exit and clean up the temporary directory stored in
/// [`TMPDIR`].
#[inline]
pub fn init() -> Result<()> {
    ctrlc::set_handler(|| {
    println!();
    tracing::info!("Interrupted by user, exiting...");

    #[allow(unused_must_use)]
    std::fs::remove_dir_all(&*TMPDIR.lock().unwrap());
    std::process::exit(0);
    })
    .context("Failed to set Ctrl-C handler")?;

    tracing::info!("SIGINT handler initialised");
    Ok(())
}
