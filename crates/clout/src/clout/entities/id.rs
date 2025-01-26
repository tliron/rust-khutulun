use {kutil_cli::debug::*, std::fmt};

//
// ID
//

/// ID.
#[derive(Clone, Debug, Debuggable, PartialEq, Eq, Hash)]
pub struct ID {
    /// Kind.
    pub kind: Kind,

    /// Namespace.
    pub namespace: Vec<String>,

    /// ID.
    pub id: String,
}

impl ID {
    /// Constructor
    pub fn new(kind: Kind, namespace: Vec<String>) -> Self {
        Self::new_for(kind, namespace, String::new())
    }

    /// Constructor
    pub fn new_for(kind: Kind, namespace: Vec<String>, id: String) -> Self {
        Self { kind, namespace, id }
    }

    /// Parse.
    pub fn parse(kind: Kind, id: &str) -> Self {
        let segments: Vec<&str> = id.split(":").collect();
        let length = segments.len();
        if length > 0 {
            Self::new_for(
                kind,
                segments[..length - 1].iter().map(|s| s.to_string()).collect(),
                segments[length - 1].into(),
            )
        } else {
            Self::new_for(kind, Vec::new(), id.into())
        }
    }

    /// Parse namespace.
    pub fn parse_namespace(namespace: &str) -> Vec<String> {
        namespace.split(":").map(|s| s.into()).collect()
    }

    /// To namespace.
    pub fn to_namespace(&self) -> Vec<String> {
        let mut namespace = self.namespace.clone();
        namespace.push(self.id.clone());
        namespace
    }
}

impl fmt::Display for ID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.namespace.is_empty() {
            write!(formatter, "{}", self.id)
        } else {
            write!(formatter, "{}{}", self.namespace.join(":") + ":", self.id)
        }
    }
}

//
// Kind
//

/// Kind.
#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    /// None.
    #[default]
    None,

    /// Type.
    Type,

    /// Node template.
    NodeTemplate,

    /// Relationship template.
    RelationshipTemplate,

    /// Node.
    Node,

    /// Relationship.
    Relationship,
}

impl fmt::Display for Kind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            match self {
                Self::None => "None",
                Self::Type => "Type",
                Self::NodeTemplate => "NodeTemplate",
                Self::RelationshipTemplate => "RelationshipTemplate",
                Self::Node => "Node",
                Self::Relationship => "Relationship",
            },
            formatter,
        )
    }
}
