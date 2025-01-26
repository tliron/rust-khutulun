use super::{super::dispatcher_bindings::GuestValueList, Value};

use std::{
    cmp::*,
    fmt::{self, Write},
    hash::*,
};

//
// List
//

/// List.
#[derive(Default, Clone)]
pub struct List {
    /// The list.
    pub value: Vec<Value>,
}

impl List {
    /// Constructor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Constructor.
    pub fn new_with<VectorT>(value: VectorT) -> Self
    where
        VectorT: Into<Vec<Value>>,
    {
        Self { value: value.into() }
    }

    /// Constructor.
    pub fn new_from<IterableT>(iterable: IterableT) -> Self
    where
        IterableT: IntoIterator<Item = Value>,
    {
        Self::new_with(Vec::from_iter(iterable))
    }
}

impl GuestValueList for List {
    fn new(list: Vec<Value>) -> Self {
        Self { value: list }
    }

    fn get(&self) -> Vec<Value> {
        self.clone().value
    }

    fn length(&self) -> u64 {
        self.value.len() as u64
    }
}

impl fmt::Display for List {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_char('[')?;

        let mut iterator = self.value.iter().peekable();
        while let Some(item) = iterator.next() {
            fmt::Display::fmt(item, formatter)?;
            if iterator.peek().is_some() {
                formatter.write_char(',')?;
            }
        }

        formatter.write_char(']')
    }
}

// Delegated

impl Hash for List {
    fn hash<HasherT>(&self, state: &mut HasherT)
    where
        HasherT: Hasher,
    {
        self.value.hash(state);
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
