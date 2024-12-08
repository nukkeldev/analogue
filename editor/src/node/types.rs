use std::fmt;

use compact_str::CompactString;

// TYPE

#[derive(Clone, PartialEq)]
pub enum Type<'a> {
    /// Unsigned `n`-bit integer
    U(usize),
    /// Fixed-sized array
    FArray(&'a Type<'a>, usize),
    /// Variably-sized array
    VArray(&'a Type<'a>),
    /// An alias to an exist type
    Alias(&'a Type<'a>),
    /// A type defined in nodes
    Defined(DefinedType<'a>),
}

impl<'a> Type<'a> {}

// DEFINED TYPE

#[derive(Debug, Clone, PartialEq)]
pub struct DefinedType<'a> {
    name: CompactString,
    fields: Vec<(CompactString, &'a Type<'a>)>,
}

impl<'a> DefinedType<'a> {
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

// DISPLAY & DEBUG

impl<'a> fmt::Display for Type<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::U(n) => write!(f, "u{n}"),
            Type::FArray(t, n) => write!(f, "{t}[{n}]"),
            Type::VArray(t) => write!(f, "{t}[]"),
            Type::Alias(t) => write!(f, "{t}"),
            Type::Defined(dt) => f.write_str(dt.get_name()),
        }
    }
}

impl<'a> fmt::Debug for Type<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl<'a> fmt::Display for DefinedType<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}
