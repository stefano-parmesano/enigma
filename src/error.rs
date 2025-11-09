use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, EnigmaError>;

#[derive(Debug)]
pub struct EnigmaError(pub String);

impl EnigmaError {
    pub(crate) fn new<T: Into<String>>(msg: T) -> Self {
        Self(msg.into())
    }
}

impl Display for EnigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for EnigmaError {}
