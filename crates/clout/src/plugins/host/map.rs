use super::{super::bindings::clout::plugins::host, host::*};

use {std::collections::*, wasmtime::component::*};

//
// Map
//

/// Map.
///
/// Importantly, complex map keys (which contain [Map] or [List](super::list::List)), are
/// identified by their representation ID, *not* their content. However, also note that
/// a [Clone] of the key maintains the representation IDs, so it would still be considered
/// the same key.
pub struct Map {
    // The map.
    value: BTreeMap<host::Value, host::Value>,
}

impl Map {
    /// Constructor.
    pub fn new(value: BTreeMap<host::Value, host::Value>) -> Self {
        Self { value }
    }
}

impl<'own> host::HostValueMap for PluginHost<'own> {
    fn new(&mut self, kv_pairs: Vec<(host::Value, host::Value)>) -> Result<Resource<Map>, wasmtime::Error> {
        let map = Map::new(BTreeMap::from_iter(kv_pairs));
        Ok(self.resources.push(map)?)
    }

    fn drop(&mut self, resource: Resource<Map>) -> Result<(), wasmtime::Error> {
        self.resources.delete(resource)?;
        Ok(())
    }

    fn get(&mut self, resource: Resource<Map>) -> Result<Vec<(host::Value, host::Value)>, wasmtime::Error> {
        let map = self.resources.get(&resource)?;
        Ok(map.value.clone().into_iter().collect())
    }

    fn length(&mut self, resource: Resource<Map>) -> Result<u64, wasmtime::Error> {
        let map = self.resources.get(&resource)?;
        Ok(map.value.len() as u64)
    }
}
