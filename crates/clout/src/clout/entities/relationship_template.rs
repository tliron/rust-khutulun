use super::{super::call::*, id::*, template::*};

use kutil_cli::debug::*;

//
// RelationshipTemplate
//

/// Relationship template.
///
/// Equivalent to TOSCA requirement.
#[derive(Clone, Debug, Debuggable)]
pub struct RelationshipTemplate {
    /// Template.
    pub template: Template,

    /// Containing source node template ID.
    pub containing_source_node_template_id: ID,

    /// Target selector.
    /// TODO: required? node/capability filter?
    pub target_selector: Call,
}
