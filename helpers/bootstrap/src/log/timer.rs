//! ## Timer
//! Module provides type and implementation for custom time formatting in tracing.

use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::format::Writer;

use colored::Colorize as _;

/// Timer for logger
/// ## Example format:
/// `24.06.2024 15:30:45`
pub(super) struct Timer;

impl FormatTime for Timer {
    fn format_time(&self, writer: &mut Writer<'_>) -> std::fmt::Result {
        write!(
            writer,
            "{} ",
            chrono::Local::now()
                .format("%d.%m.%Y %H:%M:%S")
                .to_string()
                .dimmed()
        )
    }
}