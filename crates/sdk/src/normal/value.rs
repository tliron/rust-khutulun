use super::{
    super::dispatcher_bindings::{Value, ValueList, ValueMap},
    list::*,
    map::*,
};

use {
    duplicate::*,
    ordered_float::*,
    std::{cmp::*, collections::*, fmt, hash::*},
};

// Basics

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Self::Null => Self::Null,

            Self::Integer(integer) => Self::Integer(*integer),

            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),

            Self::Float(float) => Self::Float(*float),

            Self::Boolean(boolean) => Self::Boolean(*boolean),

            Self::Text(text) => Self::Text(text.clone()),

            Self::Bytes(bytes) => Self::Bytes(bytes.clone()),

            Self::ValueList(value_list) => {
                let list: &List = value_list.get();
                list.clone().into()
            }

            Self::ValueMap(value_map) => {
                let map: &Map = value_map.get();
                map.clone().into()
            }
        }
    }
}

impl Hash for Value {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        match self {
            Self::Null => ().hash(state),

            Self::Integer(integer) => integer.hash(state),

            Self::UnsignedInteger(unsigned_integer) => unsigned_integer.hash(state),

            Self::Float(float) => OrderedFloat::from(*float).hash(state),

            Self::Boolean(boolean) => boolean.hash(state),

            Self::Text(text) => text.hash(state),

            Self::Bytes(bytes) => bytes.hash(state),

            Self::ValueList(value_list) => {
                let list: &List = value_list.get();
                list.hash(state)
            }

            Self::ValueMap(value_map) => {
                let map: &Map = value_map.get();
                map.hash(state)
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Null => matches!(other, Self::Null),

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    integer == other_integer
                } else {
                    false
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    unsigned_integer == other_unsigned_integer
                } else {
                    false
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    float == other_float
                } else {
                    false
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    boolean == other_boolean
                } else {
                    false
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    text == other_text
                } else {
                    false
                }
            }

            Self::Bytes(bytes) => {
                if let Self::Bytes(other_bytes) = other {
                    bytes == other_bytes
                } else {
                    false
                }
            }

            Self::ValueList(value_list) => {
                if let Self::ValueList(other_value_list) = other {
                    let list: &List = value_list.get();
                    let other_list: &List = other_value_list.get();
                    list == other_list
                } else {
                    false
                }
            }

            Self::ValueMap(value_map) => {
                if let Self::ValueMap(other_value_map) = other {
                    let map: &Map = value_map.get();
                    let other_map: &Map = other_value_map.get();
                    map == other_map
                } else {
                    false
                }
            }
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Self::Null => {
                if matches!(other, Self::Null) {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            Self::Integer(integer) => {
                if let Self::Integer(other_integer) = other {
                    integer.partial_cmp(other_integer)
                } else {
                    None
                }
            }

            Self::UnsignedInteger(unsigned_integer) => {
                if let Self::UnsignedInteger(other_unsigned_integer) = other {
                    unsigned_integer.partial_cmp(other_unsigned_integer)
                } else {
                    None
                }
            }

            Self::Float(float) => {
                if let Self::Float(other_float) = other {
                    float.partial_cmp(other_float)
                } else {
                    None
                }
            }

            Self::Boolean(boolean) => {
                if let Self::Boolean(other_boolean) = other {
                    boolean.partial_cmp(other_boolean)
                } else {
                    None
                }
            }

            Self::Text(text) => {
                if let Self::Text(other_text) = other {
                    text.partial_cmp(other_text)
                } else {
                    None
                }
            }

            Self::Bytes(bytes) => {
                if let Self::Bytes(other_bytes) = other {
                    bytes.partial_cmp(other_bytes)
                } else {
                    None
                }
            }

            Self::ValueList(value_list) => {
                if let Self::ValueList(other_value_list) = other {
                    let list: &List = value_list.get();
                    let other_list: &List = other_value_list.get();
                    list.partial_cmp(other_list)
                } else {
                    None
                }
            }

            Self::ValueMap(value_map) => {
                if let Self::ValueMap(other_value_map) = other {
                    let map: &Map = value_map.get();
                    let other_map: &Map = other_value_map.get();
                    map.partial_cmp(other_map)
                } else {
                    None
                }
            }
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Null => match other {
                Self::Null => Ordering::Equal,
                _ => Ordering::Less,
            },

            Value::Integer(integer) => match other {
                Self::Null => Ordering::Greater,
                Self::Integer(other_integer) => integer.cmp(other_integer),
                _ => Ordering::Less,
            },

            Value::UnsignedInteger(unsigned_integer) => match other {
                Self::Null | Self::Integer(_) => Ordering::Greater,
                Self::UnsignedInteger(other_unsigned_integer) => unsigned_integer.cmp(other_unsigned_integer),
                _ => Ordering::Less,
            },

            Value::Float(float) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) => Ordering::Greater,
                Self::Float(other_float) => OrderedFloat::from(*float).cmp(&OrderedFloat::from(*other_float)),
                _ => Ordering::Less,
            },

            Value::Boolean(boolean) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) => Ordering::Greater,
                Self::Boolean(other_boolean) => boolean.cmp(other_boolean),
                _ => Ordering::Less,
            },

            Value::Text(text) => match other {
                Self::Null | Self::Integer(_) | Self::UnsignedInteger(_) | Self::Float(_) | Self::Boolean(_) => {
                    Ordering::Greater
                }

                Self::Text(other_text) => text.cmp(other_text),

                _ => Ordering::Less,
            },

            Value::Bytes(bytes) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_) => Ordering::Greater,

                Self::Bytes(other_bytes) => bytes.cmp(other_bytes),

                _ => Ordering::Less,
            },

            Value::ValueList(value_list) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Bytes(_) => Ordering::Greater,

                Self::ValueList(other_value_list) => {
                    let list: &List = value_list.get();
                    let other_list: &List = other_value_list.get();
                    list.cmp(other_list)
                }

                _ => Ordering::Less,
            },

            Value::ValueMap(value_map) => match other {
                Self::Null
                | Self::Integer(_)
                | Self::UnsignedInteger(_)
                | Self::Float(_)
                | Self::Boolean(_)
                | Self::Text(_)
                | Self::Bytes(_)
                | Self::ValueList(_) => Ordering::Greater,

                Self::ValueMap(other_value_map) => {
                    let map: &Map = value_map.get();
                    let other_map: &Map = other_value_map.get();
                    map.cmp(other_map)
                }
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => fmt::Display::fmt("null", formatter),

            Self::Integer(integer) => fmt::Display::fmt(integer, formatter),

            Self::UnsignedInteger(unsigned_integer) => fmt::Display::fmt(unsigned_integer, formatter),

            Self::Float(float) => fmt::Display::fmt(float, formatter),

            Self::Boolean(boolean) => fmt::Display::fmt(boolean, formatter),

            Self::Text(text) => fmt::Debug::fmt(text, formatter),

            Self::Bytes(bytes) => write!(formatter, "{} bytes", bytes.len()),

            Self::ValueList(value_list) => {
                let list: &List = value_list.get();
                fmt::Display::fmt(list, formatter)
            }

            Self::ValueMap(value_map) => {
                let map: &Map = value_map.get();
                fmt::Display::fmt(map, formatter)
            }
        }
    }
}

// Conversion

impl From<List> for Value {
    fn from(list: List) -> Self {
        Self::ValueList(ValueList::new(list))
    }
}

impl From<Map> for Value {
    fn from(map: Map) -> Self {
        Self::ValueMap(ValueMap::new(map))
    }
}

// Conversion from primitives

impl From<()> for Value {
    fn from(_null: ()) -> Self {
        Self::Null
    }
}

#[duplicate_item(
  _From;
  [i64];
  [i32];
  [i16];
  [i8];
  [isize];
)]
impl From<_From> for Value {
    fn from(integer: _From) -> Self {
        Self::Integer(integer as i64)
    }
}

#[duplicate_item(
  _From;
  [u64];
  [u32];
  [u16];
  [u8];
  [usize];
)]
impl From<_From> for Value {
    fn from(unsigned_integer: _From) -> Self {
        Self::UnsignedInteger(unsigned_integer as u64)
    }
}

#[duplicate_item(
  _From;
  [f64];
  [f32];
)]
impl From<_From> for Value {
    fn from(float: _From) -> Self {
        Self::Float(float as f64)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Self::Text(string)
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Self::Text(string.into())
    }
}

impl From<Vec<u8>> for Value {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<&[u8]> for Value {
    fn from(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.into())
    }
}

impl From<Vec<Value>> for Value {
    fn from(vector: Vec<Value>) -> Self {
        List::new_with(vector).into()
    }
}

impl From<BTreeMap<Value, Value>> for Value {
    fn from(map: BTreeMap<Value, Value>) -> Self {
        Map::new_with(map).into()
    }
}
