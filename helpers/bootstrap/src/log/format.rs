//! ## Formatter
//! Code in this modules configures log format

use colored::Colorize as _;

use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime as _;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

use super::timer::Timer;
use super::visitor::TracerVisitor;

/// Type for custom log formatting
///
/// #### Example output:
/// <pre><font color="#AAAAAA">20.10.2015 18:39:36</font><font color="#284773"> ∥ </font><font color="#4E9A06">INFO</font><font color="#284773"> ∥ </font><font color="#AAAAAA">bootstrap_rs::log::init (src/log/mod.rs:16): </font>Logger initialized
/// <font color="#AAAAAA">20.10.2015 18:39:37</font><font color="#284773"> ∥ </font><font color="#C23439"><b>ERROR</b></font><font color="#284773"> ∥ </font><font color="#AAAAAA">bootstrap_rs::main (src/main.rs:37): </font>Failed to read NIaC_SELF: environment variable not found
/// </pre>
///
pub(super) struct Tracer;

impl<S, F> FormatEvent<S, F> for Tracer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    F: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, F>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();

        Timer.format_time(&mut writer)?;

        let level = match *meta.level() {
            tracing::Level::TRACE => "TRACE".purple(),
            tracing::Level::DEBUG => "DEBUG".blue(),
            tracing::Level::INFO => "INFO".green(),
            tracing::Level::WARN => "WARN".yellow().bold(),
            tracing::Level::ERROR => "ERROR".red().bold(),
        };
        write!(writer, "{sep} {level} {sep} ", sep = "∥".blue().dimmed())?;

        let mut visitor = TracerVisitor::new();
        event.record(&mut visitor);

        let spans = if let Some(scope) = ctx.event_scope() {
            let names: Vec<&str> = scope.from_root().map(|span| span.name()).collect();
            if !names.is_empty() {
                format!("::{{{}}}", names.join(", "))
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        write!(
            writer,
            "{}",
            format!(
                "{}{} ({}:{}): ",
                meta.target(),
                spans,
                meta.file().unwrap_or("/src/{unknown}.rs"),
                meta.line().map(|line| line.to_string()).unwrap_or("?".into()) //fn_name = visitor.fn_name.unwrap(),
            )
            .dimmed()
        )?;

        write!(writer, "{}", visitor.msg.unwrap().truecolor(200, 200, 200))?;

        writeln!(writer)
    }
}
