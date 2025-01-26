use super::super::{super::clout::*, bindings::clout::plugins::host};

use wasmtime_wasi::*;

//
// PluginHost
//

/// Plugins host.
pub struct PluginHost<'own> {
    /// Clout.
    pub clout: &'own Clout,

    wasi: WasiCtx,
    pub(crate) resources: ResourceTable,
}

impl<'own> PluginHost<'own> {
    /// Constructor.
    pub fn new(clout: &'own Clout) -> Self {
        let wasi = WasiCtxBuilder::new().inherit_stdout().build();
        Self { clout, wasi, resources: ResourceTable::new() }
    }
}

impl<'own> WasiView for PluginHost<'own> {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.resources
    }
}

impl<'own> host::Host for PluginHost<'own> {
    fn get_clout(&mut self, _path: Vec<host::Value>) -> Result<host::Value, wasmtime::Error> {
        Ok(self.denormalize_host_value(&self.clout.content)?)
    }
}
