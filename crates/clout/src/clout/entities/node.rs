use super::{super::store::*, debug::*, id::*, instance::*, r#type::*};

use {kutil_cli::debug::*, kutil_std::iter::*, std::*};

//
// Node
//

/// Node.
#[derive(Clone, Debug)]
pub struct Node {
    /// Instance.
    pub instance: Instance,

    /// Containing node ID.
    pub containing_node_id: Option<ID>,

    /// Contained node IDs.
    pub contained_node_ids: Vec<ID>,

    /// Outgoing relationships.
    pub outgoing_relationships: Vec<ID>,

    /// Incoming relationships.
    pub incoming_relationships: Vec<ID>,
}

impl Node {
    /// Constructor.
    pub fn new<StoreT>(namespace: Vec<String>, origin_template_id: ID, store: &mut StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::Node, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, Some(origin_template_id)))
    }

    /// Constructor.
    pub fn new_for(namespace: Vec<String>, id: String, origin_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::Node, namespace, id), origin_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, origin_template_id: Option<ID>) -> Self {
        Self {
            instance: Instance::new_with(id, origin_template_id),
            containing_node_id: None,
            contained_node_ids: Vec::new(),
            outgoing_relationships: Vec::new(),
            incoming_relationships: Vec::new(),
        }
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableNode<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableNode { node: self, store }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update<StoreT>(
        &mut self,
        library: &mut super::super::super::plugins::Library,
        plugin_name: &str,
        store: &mut StoreT,
    ) -> Result<(), super::super::ExpressionError>
    where
        StoreT: Store,
    {
        self.instance.update(library, plugin_name)?;
        store.add_node(self.clone())?;

        for node_id in &self.contained_node_ids {
            if let Some(mut node) = store.get_node(node_id)? {
                node.update(library, plugin_name, store)?;
            }
        }

        Ok(())
    }
}

//
// DebuggableNode
//

/// Debuggable node.
pub struct DebuggableNode<'own, StoreT>
where
    StoreT: Store,
{
    node: &'own Node,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableNode<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", context.theme.heading.style("Node"))?;
        write_debug_id("id", Some(&self.node.instance.id), false, writer, context)?;
        write_debug_id("node_template_id", self.node.instance.origin_template_id.as_ref(), false, writer, context)?;
        write_debug_metadata(&self.node.instance.metadata, false, writer, context)?;
        write_debug_types(&self.node.instance.type_ids, self.store, writer, context)?;
        write_debug_properties("properties", &self.node.instance.properties, false, writer, context)?;

        context.indent_into_branch(writer, true)?;
        write!(writer, "{}:", context.theme.meta.style("contained_nodes"))?;

        if self.node.contained_node_ids.is_empty() {
            write!(writer, " {}", context.theme.delimiter.style("[]"))?;
        } else {
            let item_context = context.child().increase_indentation_double_branch(true);
            for (node_id, last) in IterateWithLast::new(&self.node.contained_node_ids) {
                item_context.indent_into_double_branch(writer, last)?;
                match self.store.get_node(node_id).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                    Some(node) => {
                        node.to_debuggable(self.store)
                            .write_debug_for(writer, &item_context.child().increase_indentation_double_branch(last))?;
                    }

                    None => {
                        write_debug_id("node_id", Some(node_id), false, writer, context)?;
                    }
                }
            }
        }

        Ok(())
    }
}
