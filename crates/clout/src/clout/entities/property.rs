use super::super::call::*;

use {compris::normal::*, kutil_cli::debug::*};

//
// Property
//

/// Property.
///
/// Equivalent to TOSCA property or attribute.
#[derive(Clone, Debug, Debuggable)]
pub struct Property {
    /// Value.
    #[debuggable(option, as(debuggable))]
    pub value: Option<Value>,

    /// Updater.
    #[debuggable(option, as(debuggable))]
    pub updater: Option<Call>,

    /// Validator.
    #[debuggable(option, as(debuggable))]
    pub validator: Option<Call>,

    /// Read-only.
    #[debuggable(style(bare))]
    pub read_only: bool,
}

impl Property {
    /// Constructor.
    pub fn new(value: Option<Value>, updater: Option<Call>, validator: Option<Call>, read_only: bool) -> Self {
        Self { value, updater, validator, read_only }
    }

    /// Update.
    #[cfg(feature = "plugins")]
    pub fn update(
        &mut self,
        library: &mut super::super::super::plugins::Library,
        plugin_name: &str,
    ) -> Result<(), super::super::ExpressionError> {
        if let Some(call) = &self.updater {
            self.value = Some(call.evaluate(library, plugin_name)?);
        }
        Ok(())
    }
}
