use {thiserror::*, wasmtime::component::*};

//
// PluginError
//

/// Plugin error.
#[derive(Error, Debug)]
pub enum PluginError {
    /// Load.
    #[error("load: {0}")]
    Load(wasmtime::Error),

    /// Link.
    #[error("link: {0}")]
    Link(wasmtime::Error),

    /// Instantiate.
    #[cfg(feature = "plugins")]
    #[error("instantiate: {0}")]
    Instantiate(wasmtime::Error),

    /// Call.
    #[error("call: {0}")]
    Call(wasmtime::Error),

    /// Function.
    #[error("function: {0}")]
    Function(String),

    /// Resource.
    #[error("resource: {0}")]
    Resource(ResourceTableError),

    /// Not found.
    #[error("not found: {0}")]
    NotFound(String),
}
