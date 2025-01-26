use super::{super::entities::*, errors::*};

//
// Store
//

/// Store.
pub trait Store {
    /// Create ID.
    fn create_id(&mut self, id: &mut ID) -> Result<(), StoreError>;

    /// Get type.
    fn get_type(&self, id: &ID) -> Result<Option<Type>, StoreError>;

    /// Add type.
    fn add_type(&mut self, type_: Type) -> Result<(), StoreError>;

    /// Get node template.
    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate>, StoreError>;

    /// Add node template.
    ///
    /// Checks to make sure we aren't creating infinite nesting.
    fn add_node_template(&mut self, node_template: NodeTemplate) -> Result<(), StoreError>;

    /// Get relationship template.
    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate>, StoreError>;

    /// Add relationship template.
    fn add_relationship_template(&mut self, relationship_template: RelationshipTemplate) -> Result<(), StoreError>;

    /// Get node.
    fn get_node(&self, id: &ID) -> Result<Option<Node>, StoreError>;

    /// Add node.
    fn add_node(&mut self, node: Node) -> Result<(), StoreError>;

    /// Get relationship.
    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship>, StoreError>;

    /// Add relationship.
    fn add_relationship(&mut self, relationship: Relationship) -> Result<(), StoreError>;
}
