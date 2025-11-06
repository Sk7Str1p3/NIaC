//! ## Ctrl+C (SIGINT)
//! Functions executed if job was interrupted with SIGINT

use std::sync::Mutex;

use color_eyre::Result;
use color_eyre::eyre::Context as _;

/// Static value indicates path to $OUT.
/// This path dropped on SIGINT.
pub static TMPDIR: Mutex<String> = Mutex::new(String::new());

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
