//! ## Ctrl+C (SIGINT)
//! Functions executed if job was interrupted with SIGINT

use std::sync::Mutex;

/// Static value indicates path to $OUT.
/// This path dropped on SIGINT.
pub static TMPDIR: Mutex<String> = Mutex::new(String::new());

/// Function executed on Ctrl+C
fn handle() -> ! {
    println!();
    tracing::info!("Interrupted by user, exiting...");

    #[allow(unused_must_use)]
    std::fs::remove_dir_all(&*TMPDIR.lock().unwrap());
    std::process::exit(0);
}

#[inline]
pub fn init() {
    match ctrlc::set_handler(|| {
        handle();
    }) {
        Ok(()) => tracing::info!("SIGINT handler initialised"),
        Err(err) => tracing::error!("Failed to set Ctrl-C handler: {err}"),
    }
}
