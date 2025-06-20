use std::str::FromStr;
use curies::{Converter, error::CuriesError};

/// Error type for Identifier conversions
#[derive(Debug)]
pub enum IdentifierError {
    /// Conversion failed because the identifier is just a name
    NameNotResolvable,
    /// Error from the `curies` crate while expanding or compressing
    CurieError(CuriesError),
    /// Attempted to convert an [`Identifier`] into the wrong variant
    WrongVariant,
}

impl From<CuriesError> for IdentifierError {
    fn from(err: CuriesError) -> Self {
        IdentifierError::CurieError(err)
    }
}

/// Newtype representing a URI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri(pub String);

impl From<Uri> for Identifier {
    fn from(u: Uri) -> Self {
        Identifier::Uri(u)
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Uri {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<Identifier> for Uri {
    type Error = IdentifierError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        match value {
            Identifier::Uri(u) => Ok(u),
            _ => Err(IdentifierError::WrongVariant),
        }
    }
}

impl<'a> TryFrom<&'a Identifier> for &'a Uri {
    type Error = IdentifierError;

    fn try_from(value: &'a Identifier) -> Result<Self, Self::Error> {
        match value {
            Identifier::Uri(u) => Ok(u),
            _ => Err(IdentifierError::WrongVariant),
        }
    }
}

/// Newtype representing a CURIE.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curie(pub String);

impl From<Curie> for Identifier {
    fn from(c: Curie) -> Self {
        Identifier::Curie(c)
    }
}

impl std::fmt::Display for Curie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for Curie {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<Identifier> for Curie {
    type Error = IdentifierError;

    fn try_from(value: Identifier) -> Result<Self, Self::Error> {
        match value {
            Identifier::Curie(c) => Ok(c),
            _ => Err(IdentifierError::WrongVariant),
        }
    }
}

impl<'a> TryFrom<&'a Identifier> for &'a Curie {
    type Error = IdentifierError;

    fn try_from(value: &'a Identifier) -> Result<Self, Self::Error> {
        match value {
            Identifier::Curie(c) => Ok(c),
            _ => Err(IdentifierError::WrongVariant),
        }
    }
}

/// Enum representing either a URI, CURIE, or bare name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Identifier {
    Uri(Uri),
    Curie(Curie),
    Name(String),
}

impl Identifier {
    /// Create a new `Identifier` from a string, auto-detecting if it's a URI,
    /// CURIE, or name.
    pub fn new(s: &str) -> Self {
        if s.contains("://") {
            Identifier::Uri(Uri(s.to_string()))
        } else if s.contains(':') {
            Identifier::Curie(Curie(s.to_string()))
        } else {
            Identifier::Name(s.to_string())
        }
    }

    /// Convert this identifier to a URI using the provided prefix registry.
    ///
    /// Returns a [`Uri`] on success.
    pub fn to_uri(&self, conv: &Converter) -> Result<Uri, IdentifierError> {
        match self {
            Identifier::Uri(u) => Ok(u.clone()),
            Identifier::Curie(c) => Ok(Uri(conv.expand(&c.0)?.to_string())),
            Identifier::Name(_) => Err(IdentifierError::NameNotResolvable),
        }
    }

    /// Convert this identifier to a CURIE using the provided prefix registry
    ///
    /// Returns a [`Curie`] on success.
    pub fn to_curie(&self, conv: &Converter) -> Result<Curie, IdentifierError> {
        match self {
            Identifier::Curie(c) => Ok(c.clone()),
            Identifier::Uri(u) => Ok(Curie(conv.compress(&u.0)?.to_string())),
            Identifier::Name(_) => Err(IdentifierError::NameNotResolvable),
        }
    }
}

impl FromStr for Identifier {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Identifier::new(s))
    }
}
