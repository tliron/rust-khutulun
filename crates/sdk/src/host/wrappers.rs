use super::super::{host_bindings, normal};

// These wrappers expore only normal types and handle the conversion to and from host types

/// Get clout.
pub fn get_clout(path: Vec<normal::Value>) -> normal::Value {
    let path: Vec<host_bindings::Value> = path.into_iter().map(|v| v.into()).collect();
    let value = host_bindings::get_clout(path);
    value.into()
}
