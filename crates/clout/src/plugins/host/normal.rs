use super::{
    super::{bindings::clout::plugins::host, errors::*},
    host::*,
    list::*,
    map::*,
};

use {compris::normal, std::collections::*};

impl<'own> PluginHost<'own> {
    /// Convert a [normal::Value] to a [host::Value].
    pub fn denormalize_host_value(&mut self, value: &normal::Value) -> Result<host::Value, PluginError> {
        match value {
            normal::Value::Nothing => Ok(host::Value::Null),

            normal::Value::Null(_) => Ok(host::Value::Null),

            normal::Value::Integer(integer) => Ok(host::Value::Integer(integer.value)),

            normal::Value::UnsignedInteger(unsigned_integer) => {
                Ok(host::Value::UnsignedInteger(unsigned_integer.value))
            }

            normal::Value::Float(float) => Ok(host::Value::Float(float.into())),

            normal::Value::Boolean(boolean) => Ok(host::Value::Boolean(boolean.value)),

            normal::Value::Text(text) => Ok(host::Value::Text(text.value.clone())),

            normal::Value::Bytes(bytes) => Ok(host::Value::Bytes(bytes.value.clone())),

            normal::Value::List(list) => {
                let mut host_list = Vec::with_capacity(list.value.len());
                for value in list {
                    host_list.push(self.denormalize_host_value(value)?);
                }

                let host_list = self.resources.push(List::new(host_list)).map_err(PluginError::Resource)?;
                Ok(host::Value::ValueList(host_list))
            }

            normal::Value::Map(map) => {
                // let mut host_map = BTreeMap::with_capacity(map.value.len());
                let mut host_map = BTreeMap::new();
                for (key, value) in map {
                    host_map.insert(self.denormalize_host_value(key)?, self.denormalize_host_value(value)?);
                }

                let host_map = self.resources.push(Map::new(host_map)).map_err(PluginError::Resource)?;
                Ok(host::Value::ValueMap(host_map))
            }
        }
    }
}
