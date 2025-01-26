use super::super::{host_bindings::*, normal};

use std::collections::*;

impl From<Value> for normal::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,

            Value::Integer(integer) => Self::Integer(integer),

            Value::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),

            Value::Float(float) => Self::Float(float),

            Value::Boolean(boolean) => Self::Boolean(boolean),

            Value::Text(text) => Self::Text(text),

            Value::Bytes(bytes) => Self::Bytes(bytes),

            Value::ValueList(value_list) => {
                let list: Vec<Self> = value_list.get().into_iter().map(|v| v.into()).collect();
                list.into()
            }

            Value::ValueMap(value_map) => {
                let map: BTreeMap<Self, Self> =
                    value_map.get().into_iter().map(|(k, v)| (k.into(), v.into())).collect();
                map.into()
            }
        }
    }
}

impl From<normal::Value> for Value {
    fn from(value: normal::Value) -> Self {
        match value {
            normal::Value::Null => Self::Null,

            normal::Value::Integer(integer) => Self::Integer(integer),

            normal::Value::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(unsigned_integer),

            normal::Value::Float(float) => Self::Float(float),

            normal::Value::Boolean(boolean) => Self::Boolean(boolean),

            normal::Value::Text(text) => Self::Text(text),

            normal::Value::Bytes(bytes) => Self::Bytes(bytes),

            normal::Value::ValueList(value_list) => {
                let list: &normal::List = value_list.get();
                let list: Vec<Value> = list.value.iter().map(|v| v.clone().into()).collect();
                Self::ValueList(ValueList::new(list))
            }

            normal::Value::ValueMap(value_map) => {
                let map: &normal::Map = value_map.get();
                let kv_pairs: Vec<(Value, Value)> =
                    map.value.iter().map(|(k, v)| (k.clone().into(), v.clone().into())).collect();
                Self::ValueMap(ValueMap::new(kv_pairs))
            }
        }
    }
}
