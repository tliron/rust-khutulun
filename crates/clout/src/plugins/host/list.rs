use super::{super::bindings::clout::plugins::host, host::*};

use wasmtime::component::*;

//
// List
//

/// List.
pub struct List {
    /// The list.
    pub value: Vec<host::Value>,
}

impl List {
    /// Constructor.
    pub fn new(value: Vec<host::Value>) -> Self {
        Self { value }
    }
}

impl<'own> host::HostValueList for PluginHost<'own> {
    fn new(&mut self, list: Vec<host::Value>) -> Result<Resource<List>, wasmtime::Error> {
        let list = List::new(list);
        Ok(self.resources.push(list)?)
    }

    fn drop(&mut self, resource: Resource<List>) -> Result<(), wasmtime::Error> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<List>) -> Result<Vec<host::Value>, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.value.clone())
    }

    fn length(&mut self, resource: Resource<List>) -> Result<u64, wasmtime::Error> {
        let list = self.resources.get(&resource)?;
        Ok(list.value.len() as u64)
    }
}
