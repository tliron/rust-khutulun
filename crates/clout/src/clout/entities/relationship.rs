use super::{id::*, instance::*};

use kutil_cli::debug::*;

//
// Relationship
//

/// Relationship.
#[derive(Clone, Debug, Debuggable)]
pub struct Relationship {
    /// Instance.
    pub instance: Instance,

    /// Source node ID.
    pub source_node_id: ID,

    /// Target node ID.
    pub target_node_id: ID,
}
