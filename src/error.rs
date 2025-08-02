use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct EnigmaError<'a>(pub &'a str);

impl<'a> Display for EnigmaError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Error for EnigmaError<'a> {}
