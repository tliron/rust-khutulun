use super::super::call::*;

use kutil_cli::debug::*;

//
// EventHandler
//

/// Event handler.
///
/// Equivalent to TOSCA operation or notification.
#[derive(Clone, Debug, Debuggable)]
pub struct EventHandler {
    /// Event ID.
    pub event_id: String,

    /// Call.
    pub call: Call,
}
