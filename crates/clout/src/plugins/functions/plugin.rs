use super::super::{bindings, environment::*, errors::*, host::*};

use {
    compris::normal::*,
    std::path,
    wasmtime::{component::*, Store},
};

//
// FunctionsPlugin
//

/// Clout functions plugin.
pub struct FunctionsPlugin<'own> {
    pub(crate) store: Store<PluginHost<'own>>,
    pub(crate) functions: bindings::Functions,
}

impl<'own> FunctionsPlugin<'own> {
    /// Constructor.
    pub fn new(environment: &'own Environment, bytes: &[u8]) -> Result<Self, PluginError> {
        let component = Component::from_binary(&environment.engine, bytes).map_err(PluginError::Load)?;
        Self::new_with(environment, component)
    }

    /// Constructor.
    pub fn new_with(environment: &'own Environment, component: Component) -> Result<Self, PluginError> {
        // Linker
        let mut linker = Linker::new(&environment.engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(PluginError::Link)?;
        bindings::Functions::add_to_linker(&mut linker, |state: &mut PluginHost| state).map_err(PluginError::Link)?;

        // Store
        let mut store = Store::new(&environment.engine, PluginHost::new(&environment.clout));

        // Bindings
        let functions =
            bindings::Functions::instantiate(&mut store, &component, &linker).map_err(PluginError::Instantiate)?;

        Ok(Self { store, functions })
    }

    /// Constructor.
    pub fn new_from_file<PathT>(environment: &'own Environment, path: PathT) -> Result<Self, PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let component = Component::from_file(&environment.engine, path).map_err(PluginError::Load)?;
        Self::new_with(environment, component)
    }

    /// Dispatch.
    pub fn dispatch(&mut self, name: &str) -> Result<Value, PluginError> {
        let value = self
            .functions
            .clout_plugins_dispatcher()
            .call_dispatch(&mut self.store, name, &Vec::new())
            .map_err(PluginError::Call)?
            .map_err(PluginError::Function)?;

        Ok(self.normalize_guest_value(value)?)
    }
}
