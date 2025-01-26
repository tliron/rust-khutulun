use super::{super::store::*, debug::*, id::*, metadata::*};

use {kutil_cli::debug::*, kutil_std::iter::*, std::io};

//
// Type
//

/// Type.
#[derive(Clone, Debug)]
pub struct Type {
    /// ID.
    pub id: ID,

    /// Metadata.
    pub metadata: Metadata,

    /// Parent type IDs.
    pub parent_type_ids: Vec<ID>,

    /// Child type IDs.
    pub child_type_ids: Vec<ID>,
}

impl Type {
    /// Constructor.
    pub fn new_for(namespace: Vec<String>, id: String) -> Self {
        Self::new_with(ID::new_for(Kind::Type, namespace, id))
    }

    /// Constructor.
    pub fn new_with(id: ID) -> Self {
        Self { id, metadata: Metadata::new(), parent_type_ids: Vec::new(), child_type_ids: Vec::new() }
    }

    /// To [Debuggable].
    pub fn to_debuggable<'own, StoreT>(&'own self, store: &'own StoreT) -> DebuggableType<'own, StoreT>
    where
        StoreT: Store,
    {
        DebuggableType { type_: self, store }
    }
}

//
// DebuggableType
//

/// Debuggable type.
#[allow(unused)]
pub struct DebuggableType<'own, StoreT>
where
    StoreT: Store,
{
    type_: &'own Type,
    store: &'own StoreT,
}

impl<'own, StoreT> Debuggable for DebuggableType<'own, StoreT>
where
    StoreT: Store,
{
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        write!(writer, "{}", context.theme.heading.style("Type"))?;

        context.indent_into_branch(writer, false)?;
        write!(writer, "{}: {}", context.theme.meta.style("id"), context.theme.name.style(&self.type_.id))?;

        write_debug_metadata(&self.type_.metadata, true, writer, context)?;

        Ok(())
    }
}

/// Write debug types.
pub fn write_debug_types<WriteT, StoreT>(
    type_ids: &Vec<ID>,
    store: &StoreT,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
    StoreT: Store,
{
    context.indent_into_branch(writer, false)?;
    write!(writer, "{}:", context.theme.meta.style("types"))?;

    if type_ids.is_empty() {
        write!(writer, " {}", context.theme.delimiter.style("[]"))?;
    } else {
        let item_context = context.child().increase_indentation_branch(false);
        for (type_id, last) in IterateWithLast::new(type_ids) {
            item_context.indent_into_branch(writer, last)?;
            match store.get_type(type_id).map_err(|e| io::Error::new(io::ErrorKind::Other, e))? {
                Some(type_) => {
                    type_
                        .to_debuggable(store)
                        .write_debug_for(writer, &item_context.child().increase_indentation_branch(last))?;
                }

                None => {
                    write!(writer, "{}: {}", context.theme.meta.style("type_id"), type_id)?;
                }
            }
        }
    }

    Ok(())
}
