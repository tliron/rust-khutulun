use super::{super::store::*, debug::*, id::*, node::*, r#type::*, template::*};

use {kutil_cli::debug::*, kutil_std::iter::*, std::io};

//
// NodeTemplate
//

/// Node template.
///
/// Equivalent to TOSCA service template, node template, or capability definition.
#[derive(Clone, Debug)]
pub struct NodeTemplate {
    /// Template.
    pub template: Template,

    /// Containing node template ID.
    pub containing_node_template_id: Option<ID>,

    /// Contained node template IDs.
    pub contained_node_template_ids: Vec<ID>,

    /// Relationship templates.
    pub relationship_template_ids: Vec<ID>,
}

impl NodeTemplate {
    /// Constructor.
    pub fn new<StoreT>(namespace: Vec<String>, store: &mut StoreT) -> Result<Self, StoreError>
    where
        StoreT: Store,
    {
        let mut id = ID::new(Kind::NodeTemplate, namespace);
        store.create_id(&mut id)?;
        Ok(Self::new_with(id, None))
    }

    /// Constructor.
    pub fn new_for(namespace: Vec<String>, id: String, containing_node_template_id: Option<ID>) -> Self {
        Self::new_with(ID::new_for(Kind::NodeTemplate, namespace, id), containing_node_template_id)
    }

    /// Constructor.
    pub fn new_with(id: ID, containing_node_template_id: Option<ID>) -> Self {
        Self {
            template: Template::new(id),
            containing_node_template_id,
            contained_node_template_ids: Vec::new(),
            relationship_template_ids: Vec::new(),
        }
    }

    /// Instantiate.
    pub fn instantiate<StoreT>(&self, namespace: &Vec<String>, store: &mut StoreT) -> Result<ID, StoreError>
    where
        StoreT: Store,
    {
        self.instantiate_for(namespace, None, store)
    }

    /// Instantiate.
    pub fn instantiate_for<StoreT>(
        &self,
        namespace: &Vec<String>,
        containing_node_id: Option<ID>,
        store: &mut StoreT,
    ) -> Result<ID, StoreError>
    where
        StoreT: Store,
    {
        let mut node = Node {
            instance: self.template.instantiate(Kind::Node, namespace, store)?,
            containing_node_id: containing_node_id.clone(),
            contained_node_ids: Vec::with_capacity(self.contained_node_template_ids.len()),
            outgoing_relationships: Vec::new(),
            incoming_relationships: Vec::new(),
        };

        let id = node.instance.id.clone();

        for contained_node_template_id in &self.contained_node_template_ids {
            if let Some(contained_node_template) = store.get_node_template(contained_node_template_id)? {
                let contained_node_id = contained_node_template.instantiate_for(namespace, Some(id.clone()), store)?;
                node.contained_node_ids.push(contained_node_id);
            }
        }

        store.add_node(node)?;

        Ok(id)
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableNodeTemplate<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableNodeTemplate { node_template: self, store }
    }
}

//
// DebuggableNodeTemplate
//

/// Debuggable node template.
pub struct DebuggableNodeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    node_template: &'own NodeTemplate,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableNodeTemplate<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", context.theme.heading.style("NodeTemplate"))?;
        write_debug_id("id", Some(&self.node_template.template.id), false, writer, context)?;
        write_debug_metadata(&self.node_template.template.metadata, false, writer, context)?;
        write_debug_types(&self.node_template.template.type_ids, self.store, writer, context)?;
        write_debug_properties(
            "property_templates",
            &self.node_template.template.property_templates,
            false,
            writer,
            context,
        )?;

        context.indent_into_branch(writer, true)?;
        write!(writer, "{}:", context.theme.meta.style("contained_node_templates"))?;

        if self.node_template.contained_node_template_ids.is_empty() {
            write!(writer, " {}", context.theme.delimiter.style("[]"))?;
        } else {
            let item_context = context.child().increase_indentation_double_branch(true);
            for (node_template_id, last) in IterateWithLast::new(&self.node_template.contained_node_template_ids) {
                item_context.indent_into_double_branch(writer, last)?;
                match self
                    .store
                    .get_node_template(node_template_id)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
                {
                    Some(node_template) => {
                        node_template
                            .to_debuggable(self.store)
                            .write_debug_for(writer, &item_context.child().increase_indentation_double_branch(last))?;
                    }

                    None => {
                        write_debug_id("node_template_id", Some(node_template_id), false, writer, context)?;
                    }
                }
            }
        }

        Ok(())
    }
}
