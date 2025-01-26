use super::store::*;

use thiserror::*;

//
// Expression
//

/// Syntax error.
#[derive(Error, Debug)]
pub enum ExpressionError {
    /// Store.
    #[error("store: {0}")]
    Store(#[from] StoreError),

    /// Plugin.
    #[cfg(feature = "plugins")]
    #[error("plugin: {0}")]
    Plugin(#[from] super::super::plugins::PluginError),
}
