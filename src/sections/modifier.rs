use crate::error::EnigmaError;
use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FormattedAccessModifier {
    pub access_modifier: Option<AccessModifier>,
}

impl Display for FormattedAccessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.access_modifier
                .as_ref()
                .map(|am| format!(" ACC:{}", am))
                .unwrap_or_default()
        )
    }
}

impl FromStr for FormattedAccessModifier {
    type Err = EnigmaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix(" ACC:") {
            Some(am) => Ok(Self {
                access_modifier: Some(AccessModifier::from_str(am)?),
            }),
            None => match s {
                "" => Ok(Self::default()),
                _ => Err(EnigmaError(format!(
                    "'{}' is not a valid formatted access modifier",
                    s
                ))),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessModifier {
    Unchanged,
    Public,
    Protected,
    Private,
}

impl FromStr for AccessModifier {
    type Err = EnigmaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UNCHANGED" => Ok(Self::Unchanged),
            "PUBLIC" => Ok(Self::Public),
            "PROTECTED" => Ok(Self::Protected),
            "PRIVATE" => Ok(Self::Private),
            _ => Err(EnigmaError::new("Invalid access modifier!")),
        }
    }
}

impl Display for AccessModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Unchanged => "UNCHANGED",
                Self::Public => "PUBLIC",
                Self::Protected => "PROTECTED",
                Self::Private => "PRIVATE",
            }
        )
    }
}
