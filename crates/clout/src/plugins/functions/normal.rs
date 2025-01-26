use super::super::{bindings::exports::clout::plugins::dispatcher, errors::*, functions::*};

use compris::normal;

impl<'own> FunctionsPlugin<'own> {
    /// Convert a [dispatcher::Value] to a [normal::Value].
    pub fn normalize_guest_value(&mut self, value: dispatcher::Value) -> Result<normal::Value, PluginError> {
        match value {
            dispatcher::Value::Null => Ok(normal::Null::new().into()),

            dispatcher::Value::Integer(integer) => Ok(normal::Integer::new(integer).into()),

            dispatcher::Value::UnsignedInteger(unsigned_integer) => {
                Ok(normal::UnsignedInteger::new(unsigned_integer).into())
            }

            dispatcher::Value::Float(float) => Ok(normal::Float::new(float).into()),

            dispatcher::Value::Boolean(boolean) => Ok(normal::Boolean::new(boolean).into()),

            dispatcher::Value::Text(text) => Ok(normal::Text::new(text).into()),

            dispatcher::Value::Bytes(bytes) => Ok(normal::Bytes::new(bytes).into()),

            dispatcher::Value::ValueList(resource) => {
                let value_list = self.functions.clout_plugins_dispatcher().value_list();
                let from_list = value_list.call_get(&mut self.store, resource).unwrap();
                resource.resource_drop(&mut self.store).map_err(PluginError::Call)?;

                let mut list = normal::List::new();
                for value in from_list {
                    list.value.push(self.normalize_guest_value(value)?);
                }

                Ok(list.into())
            }

            dispatcher::Value::ValueMap(resource) => {
                let value_map = self.functions.clout_plugins_dispatcher().value_map();
                let from_map = value_map.call_get(&mut self.store, resource).unwrap();
                resource.resource_drop(&mut self.store).map_err(PluginError::Call)?;

                let mut map = normal::Map::new();
                for (key, value) in from_map {
                    map.value.insert(self.normalize_guest_value(key)?, self.normalize_guest_value(value)?);
                }

                Ok(map.into())
            }
        }
    }
}
