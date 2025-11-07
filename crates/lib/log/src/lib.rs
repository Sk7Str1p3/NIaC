//! # Logging setup
//! Custom format for `tracing` logger.
//!
//! Implement custom formatting

mod format;
mod timer;
mod visitor;

use color_eyre::Result;
use color_eyre::eyre::Context as _;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt as _;

/// Initializes logger with custom format
/// ### Example output:
#[doc = r##"
<pre>
 <font color="#AAAAAA">20.10.2015 18:39:36</font><font color="#284773"> ∥ </font><font color="#4E9A06">INFO</font><font color="#284773"> ∥ </font><font color="#AAAAAA">bootstrap_rs::log::init (src/log/mod.rs:16): </font>Logger initialized
 <font color="#AAAAAA">20.10.2015 18:39:37</font><font color="#284773"> ∥ </font><font color="#C23439"><b>ERROR</b></font><font color="#284773"> ∥ </font><font color="#AAAAAA">bootstrap_rs::main (src/main.rs:37): </font>Failed to read NIaC_SELF: environment variable not found
</pre>
"##]
#[inline]
pub fn install() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .event_format(format::Tracer)
        .finish()
        .with(ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber).context("Failed to set logger")?;

    tracing::info!("Logger initialized");
    Ok(())
}
