use super::{id::*, metadata::*, property::*};

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{collections::*, io},
};

/// Write debug metadata.
pub fn write_debug_metadata<WriteT>(
    metadata: &Metadata,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::write_debug_field("metadata", last, writer, context, |writer, context| -> io::Result<()> {
        metadata.write_debug_for(writer, context)
    })
}

/// Write debug ID.
pub fn write_debug_id<WriteT>(
    name: &str,
    id: Option<&ID>,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    context.indent_into_branch(writer, last)?;
    write!(writer, "{}: ", context.theme.meta.style(name))?;
    match id {
        Some(id) => write!(writer, "{}", context.theme.name.style(id)),
        None => write!(writer, "{}", context.theme.bare.style("None")),
    }
}

/// Write debug ID.
pub fn write_debug_properties<WriteT>(
    name: &str,
    properties: &BTreeMap<String, Property>,
    last: bool,
    writer: &mut WriteT,
    context: &DebugContext,
) -> io::Result<()>
where
    WriteT: io::Write,
{
    utils::write_debug_field(name, last, writer, context, |writer, context| -> io::Result<()> {
        if properties.is_empty() {
            context.separate(writer)?;
            write!(writer, "{}", context.theme.delimiter.style("{}"))?;
        } else {
            for ((name, property), last) in IterateWithLast::new(properties) {
                context.indent_into_branch(writer, last)?;
                write!(writer, "{}:", context.theme.meta.style(name))?;
                property
                    .write_debug_for(writer, &context.child().with_inline(true).increase_indentation_branch(last))?;
            }
        }

        Ok(())
    })
}
