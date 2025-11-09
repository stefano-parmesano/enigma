use crate::Class;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Content<'a> {
    pub classes: Vec<Class<'a>>,
}
