use super::{environment::*, errors::*, functions::*};

use std::{collections::*, path};

//
// Library
//

/// Plugin library.
pub struct Library<'own> {
    /// Environment.
    pub environment: &'own Environment<'own>,

    /// Functions.
    pub functions: HashMap<String, FunctionsPlugin<'own>>,
}

impl<'own> Library<'own> {
    /// Constructor.
    pub fn new(environment: &'own Environment<'own>) -> Self {
        Self { environment, functions: HashMap::new() }
    }

    /// Add a [FunctionsPlugin].
    pub fn add_functions(&mut self, plugin_name: &str, bytes: &[u8]) -> Result<(), PluginError> {
        let functions = FunctionsPlugin::new(self.environment, bytes)?;
        self.functions.insert(plugin_name.into(), functions);
        Ok(())
    }

    /// Load a [FunctionsPlugin].
    pub fn load_functions<PathT>(&mut self, plugin_name: &str, path: PathT) -> Result<(), PluginError>
    where
        PathT: AsRef<path::Path>,
    {
        let functions = FunctionsPlugin::new_from_file(self.environment, path)?;
        self.functions.insert(plugin_name.into(), functions);
        Ok(())
    }

    /// Get a [FunctionsPlugin].
    pub fn get_functions(&mut self, plugin_name: &str) -> Result<&mut FunctionsPlugin<'own>, PluginError> {
        match self.functions.get_mut(plugin_name) {
            Some(functions) => Ok(functions),
            None => Err(PluginError::NotFound(plugin_name.into())),
        }
    }
}
