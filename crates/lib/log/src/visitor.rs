//! ## Visitor
//! Module provides visitor type for logger for custom
//! fields.

use tracing::field::Visit;

/// Visitor type for logger
pub(crate) struct TracerVisitor {
    /// Message of the event
    pub(crate) msg: Option<String>
}

impl TracerVisitor {
    /// Creates new visitor with empty fields
    pub(crate) fn new() -> Self { Self { msg: None } }
}

impl Visit for TracerVisitor {
    fn record_str(
        &mut self,
        field: &tracing::field::Field,
        value: &str
    ) {
        if field.name() == "message" {
            self.msg = Some(value.to_owned())
        };
    }

    fn record_debug(
        &mut self,
        field: &tracing::field::Field,
        value: &dyn std::fmt::Debug
    ) {
        if field.name() == "message" {
            self.msg = Some(format!("{value:?}"))
        };
    }
}
