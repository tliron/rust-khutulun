use super::{super::entities::*, errors::*, store::*};

use {
    std::{collections::*, sync::atomic::*},
    tracing::info,
};

//
// InMemoryStore
//

/// In-memory store.
pub struct InMemoryStore {
    next_id: AtomicU64,

    types: HashMap<ID, Type>,
    node_templates: HashMap<ID, NodeTemplate>,
    relationship_templates: HashMap<ID, RelationshipTemplate>,
    nodes: HashMap<ID, Node>,
    relationships: HashMap<ID, Relationship>,
}

impl InMemoryStore {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(1),
            types: HashMap::new(),
            node_templates: HashMap::new(),
            relationship_templates: HashMap::new(),
            nodes: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
}

impl Store for InMemoryStore {
    fn create_id(&mut self, id: &mut ID) -> Result<(), StoreError> {
        let next_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        id.id = next_id.to_string();
        info!("create_id: {}", id);
        Ok(())
    }

    fn get_type(&self, id: &ID) -> Result<Option<Type>, StoreError> {
        info!("get_type: {}", id);
        if id.kind != Kind::Type {
            return Err(StoreError::ID(format!("kind is not Type: {}", id.kind)));
        }
        Ok(self.types.get(id).map(|t| t.clone()))
    }

    fn add_type(&mut self, type_: Type) -> Result<(), StoreError> {
        info!("add_type: {}", type_.id);
        if type_.id.kind != Kind::Type {
            return Err(StoreError::ID(format!("kind is not Type: {}", type_.id.kind)));
        }
        self.types.insert(type_.id.clone(), type_);
        Ok(())
    }

    fn get_node_template(&self, id: &ID) -> Result<Option<NodeTemplate>, StoreError> {
        info!("get_node_template: {}", id);
        if id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", id.kind)));
        }
        Ok(self.node_templates.get(id).map(|n| n.clone()))
    }

    fn add_node_template(&mut self, node_template: NodeTemplate) -> Result<(), StoreError> {
        info!("add_node_template: {}", node_template.template.id);
        if node_template.template.id.kind != Kind::NodeTemplate {
            return Err(StoreError::ID(format!("kind is not NodeTemplate: {}", node_template.template.id.kind)));
        }
        self.node_templates.insert(node_template.template.id.clone(), node_template);
        Ok(())
    }

    fn get_relationship_template(&self, id: &ID) -> Result<Option<RelationshipTemplate>, StoreError> {
        info!("get_relationship_template: {}", id);
        if id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!("kind is not RelationshipTemplate: {}", id.kind)));
        }
        Ok(self.relationship_templates.get(id).map(|r| r.clone()))
    }

    fn add_relationship_template(&mut self, relationship_template: RelationshipTemplate) -> Result<(), StoreError> {
        info!("add_relationship_template: {}", relationship_template.template.id);
        if relationship_template.template.id.kind != Kind::RelationshipTemplate {
            return Err(StoreError::ID(format!(
                "kind is not RelationshipTemplate: {}",
                relationship_template.template.id.kind
            )));
        }
        self.relationship_templates.insert(relationship_template.template.id.clone(), relationship_template);
        Ok(())
    }

    fn get_node(&self, id: &ID) -> Result<Option<Node>, StoreError> {
        info!("get_node: {}", id);
        if id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", id.kind)));
        }
        Ok(self.nodes.get(id).map(|n| n.clone()))
    }

    fn add_node(&mut self, node: Node) -> Result<(), StoreError> {
        info!("add_node: {}", node.instance.id);
        if node.instance.id.kind != Kind::Node {
            return Err(StoreError::ID(format!("kind is not Node: {}", node.instance.id.kind)));
        }
        self.nodes.insert(node.instance.id.clone(), node);
        Ok(())
    }

    fn get_relationship(&self, id: &ID) -> Result<Option<Relationship>, StoreError> {
        info!("get_relationship: {}", id);
        if id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", id.kind)));
        }
        Ok(self.relationships.get(id).map(|r| r.clone()))
    }

    fn add_relationship(&mut self, relationship: Relationship) -> Result<(), StoreError> {
        info!("add_relationship: {}", relationship.instance.id);
        if relationship.instance.id.kind != Kind::Relationship {
            return Err(StoreError::ID(format!("kind is not Relationship: {}", relationship.instance.id.kind)));
        }
        self.relationships.insert(relationship.instance.id.clone(), relationship);
        Ok(())
    }
}
