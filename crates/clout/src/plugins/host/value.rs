use super::super::bindings::clout::plugins::host::*;

use {
    ordered_float::*,
    std::{cmp::*, hash::*},
    wasmtime::component::Resource,
};

impl Clone for Value {
    /// Clone resource.
    fn clone(&self) -> Value {
        match self {
            Self::Null => Self::Null,
            Self::Integer(integer) => Self::Integer(*integer),
            Self::UnsignedInteger(unsigned_integer) => Self::UnsignedInteger(*unsigned_integer),
            Self::Float(float) => Self::Float(*float),
            Self::Boolean(boolean) => Self::Boolean(*boolean),
            Self::Text(text) => Self::Text(text.clone()),
            Self::Bytes(bytes) => Self::Bytes(bytes.clone()),
            // TODO: own or borrow?
            Self::ValueList(resource) => Self::ValueList(Resource::new_own(resource.rep())),
            Self::ValueMap(resource) => Self::ValueMap(Resource::new_own(resource.rep())),
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
            Self::ValueList(value_list) => value_list.rep().hash(state),
            Self::ValueMap(value_map) => value_map.rep().hash(state),
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
                // Important: this is not a content comparison!
                if let Self::ValueList(other_value_list) = other {
                    value_list.rep() == other_value_list.rep()
                } else {
                    false
                }
            }

            Self::ValueMap(value_map) => {
                // Important: this is not a content comparison!
                if let Self::ValueMap(other_value_map) = other {
                    value_map.rep() == other_value_map.rep()
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
                // Important: this is not a content comparison!
                if let Self::ValueList(other_value_list) = other {
                    if value_list.rep() == other_value_list.rep() {
                        Some(Ordering::Equal)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            Self::ValueMap(value_map) => {
                // Important: this is not a content comparison!
                if let Self::ValueMap(other_value_map) = other {
                    if value_map.rep() == other_value_map.rep() {
                        Some(Ordering::Equal)
                    } else {
                        None
                    }
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

                // Important: this is not a content comparison!
                Self::ValueList(other_value_list) => value_list.rep().cmp(&other_value_list.rep()),

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

                // Important: this is not a content comparison!
                Self::ValueMap(other_value_map) => value_map.rep().cmp(&other_value_map.rep()),
            },
        }
    }
}
