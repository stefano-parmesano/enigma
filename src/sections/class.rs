use crate::ClassSubSection;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Class<'a> {
    pub class_name_a: &'a str,
    pub class_name_b: Option<&'a str>,
    pub formatted_access_modifier: Option<&'a str>,
    pub class_sub_sections: Vec<ClassSubSection<'a>>,
}
