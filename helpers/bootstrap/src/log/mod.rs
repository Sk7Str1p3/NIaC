//! # Logging setup
//! Custom format for `tracing` logger.
//!
//! Implement custom formatting

mod format;
mod timer;
mod visitor;

use color_eyre::Result;
use color_eyre::eyre::Context;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
/// Initializes logger with custom format
#[inline]
pub fn init() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .event_format(format::Tracer)
        .finish()
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).context("Failed to set logger")?;

    tracing::info!("Logger initialized");
    Ok(())
}
