pub mod error;
mod sections;

pub use sections::*;

/// This function parses the entire string from a tiny file
pub fn from_str(s: &str) -> Result<File<'_>, error::EnigmaError> {
    File::try_from(s)
}
