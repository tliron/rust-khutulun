use compris::normal::*;

//
// Clout
//

/// Clout.
#[derive(Clone)]
pub struct Clout {
    /// Content.
    pub content: Value,
}

impl Clout {
    /// Constructor.
    pub fn new(content: Value) -> Self {
        Self { content }
    }
}
