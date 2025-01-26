use super::{super::store::*, event_handler::*, id::*, instance::*, metadata::*, property::*};

use {kutil_cli::debug::*, std::collections::*};

//
// Template
//

/// Template.
#[derive(Clone, Debug, Debuggable)]
pub struct Template {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Type IDs.
    pub type_ids: Vec<ID>,

    /// Property templates.
    pub property_templates: BTreeMap<String, Property>,

    /// Event handlers.
    pub event_handlers: Vec<EventHandler>,
}

impl Template {
    /// Constructor.
    pub fn new(id: ID) -> Self {
        Self {
            id,
            metadata: Metadata::new(),
            type_ids: Vec::new(),
            property_templates: BTreeMap::new(),
            event_handlers: Vec::new(),
        }
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        kind: Kind,
        namespace: &Vec<String>,
        store: &mut StoreT,
    ) -> Result<Instance, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(kind, namespace.clone());
        store.create_id(&mut id)?;

        let mut instance = Instance::new_with(id, Some(self.id.clone()));
        instance.metadata = self.metadata.clone();
        instance.type_ids = self.type_ids.clone();
        instance.properties = self.property_templates.clone();

        Ok(instance)
    }
}
