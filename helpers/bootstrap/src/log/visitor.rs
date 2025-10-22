//! ## Visitor
//! Module provides visitor type for logger for custom fields.

use tracing::field::Visit;

/// Visitor type for logger
pub(super) struct TracerVisitor {
    /// Message of the event
    pub(super) msg: Option<String>,
}

impl TracerVisitor {
    /// Creates new visitor with empty fields
    pub(super) fn new() -> Self {
        Self {
            msg: None,
        }
    }
}

impl Visit for TracerVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "message" => self.msg = Some(value.to_owned()),
            _ => (),
        };
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        match field.name() {
            "message" => self.msg = Some(format!("{value:?}")),
            _ => (),
        };
    }
}
