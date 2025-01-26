use super::super::clout::*;

use wasmtime::*;

//
// Environment
//

/// Plugin environment.
pub struct Environment<'own> {
    /// Wasmtime engine.
    pub engine: Engine,

    /// Clout.
    pub clout: &'own Clout,
}

impl<'own> Environment<'own> {
    /// Constructor.
    pub fn new(clout: &'own Clout) -> Self {
        Self { engine: Engine::default(), clout }
    }
}
