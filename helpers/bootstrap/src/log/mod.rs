//! # Logging setup
//! Custom format for `tracing` logger.
//!
//! Implement custom formatting

mod format;
mod timer;
mod visitor;

/// Initializes logger with custom format
#[inline]
pub fn init() {
    tracing_subscriber::fmt()
        .event_format(format::Tracer)
        .init();
    tracing::info!("Logger initialized");
}
