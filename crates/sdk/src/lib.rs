#![warn(missing_docs)]

/*!
Plugin SDK.
*/

mod macros;

/// WIT bindings.
pub mod bindings;

/// Plugin host.
pub mod host;

/// Normal value types.
pub mod normal;

#[allow(unused_imports)]
pub use bindings::{
    clout::plugins::host as host_bindings, export_dispatcher,
    exports::clout::plugins::dispatcher as dispatcher_bindings,
};
