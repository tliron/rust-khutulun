use super::expression::*;

use {
    kutil_cli::debug::*,
    kutil_std::iter::*,
    std::{fmt, io},
};

#[cfg(feature = "plugins")]
use {
    super::{super::plugins::*, errors::*},
    compris::normal::*,
};

//
// Call
//

/// Call.
#[derive(Default, Clone, Debug)]
pub struct Call {
    /// Function name.
    pub name: String,

    /// Arguments.
    pub arguments: Vec<Expression>,
}

impl Call {
    /// Constructor.
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), arguments: Vec::new() }
    }

    /// Evaluate.
    #[cfg(feature = "plugins")]
    pub fn evaluate(&self, library: &mut Library, plugin_name: &str) -> Result<Value, ExpressionError> {
        Ok(library.get_functions(plugin_name)?.dispatch(&self.name)?)
    }
}

impl Debuggable for Call {
    fn write_debug_for<WriteT>(&self, writer: &mut WriteT, context: &DebugContext) -> io::Result<()>
    where
        WriteT: io::Write,
    {
        context.separate(writer)?;
        write!(writer, "{}{}", context.theme.name.style(&self.name), context.theme.delimiter.style("("))?;

        let child_context = &context.child().with_format(DebugFormat::Compact).with_separator(false);
        for (argument, last) in IterateWithLast::new(&self.arguments) {
            argument.write_debug_for(writer, child_context)?;
            if !last {
                write!(writer, "{}", context.theme.delimiter.style(","))?;
            }
        }

        write!(writer, "{}", context.theme.delimiter.style(")"))
    }
}

impl fmt::Display for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}(", self.name)?;

        for (argument, last) in IterateWithLast::new(&self.arguments) {
            fmt::Display::fmt(argument, formatter)?;
            if !last {
                write!(formatter, ",")?;
            }
        }

        write!(formatter, ")")
    }
}
