use crate::{Content, error::EnigmaError};

/// Entire parsed tiny file
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct File<'a> {
    pub content: Content<'a>,
}

impl<'a> File<'a> {
    pub fn new(content: Content<'a>) -> Self {
        Self { content }
    }

    fn from_str(s: &'a str) -> Result<Self, EnigmaError> {
        let mut lines = s.lines();
        let mut file = Self::default();
        while let Some(line) = lines.next() {
            let leading_tabs = line.chars().take_while(|c| *c == '\t').count();
            let mut tokens = line.trim_ascii().split('\t');
            let identifier = tokens
                .next()
                .filter(|s| !s.is_empty())
                .ok_or(EnigmaError::new("Line empty"))?;
            match identifier {
                "CLASS" => {}
                "FIELD" => {}
                "METHOD" => {}
                "ARG" => {}
                id => {
                    // File comments
                    if !id.starts_with(['#', '/', '*']) {
                        return Err(EnigmaError(format!(
                            "Wrong identifier (Tabs: {}, Identifier: {})",
                            leading_tabs, id
                        )));
                    }
                }
            }
        }
        Err(EnigmaError::new("Header not found!"))
    }
}

impl<'a> TryFrom<&'a str> for File<'a> {
    type Error = EnigmaError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}
