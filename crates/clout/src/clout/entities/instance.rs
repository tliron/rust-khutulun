use super::{id::*, metadata::*, property::*};

use std::collections::*;

//
// Instance
//

/// Instance.
#[derive(Clone, Debug)]
pub struct Instance {
    /// ID.
    pub id: ID,

    /// Origin template ID.
    pub origin_template_id: Option<ID>,

    /// Metadata.
    pub metadata: Metadata,

    /// Type IDs.
    pub type_ids: Vec<ID>,

    /// Properties.
    pub properties: BTreeMap<String, Property>,
}

impl Instance {
    /// Constructor.
    pub fn new_for(kind: Kind, namespace: Vec<String>, id: String, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(kind, namespace, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self { id, origin_template_id, metadata: Metadata::new(), type_ids: Vec::new(), properties: BTreeMap::new() }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update(
        &mut self,
        library: &mut super::super::super::plugins::Library,
        plugin_name: &str,
    ) -> Result<(), super::super::ExpressionError> {
        for property in self.properties.values_mut() {
            property.update(library, plugin_name)?;
        }
        Ok(())
    }
}
