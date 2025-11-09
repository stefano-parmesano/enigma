use crate::FormattedAccessModifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ClassSubSection<'a> {
    CLASS {
        class_name_a: &'a str,
        class_name_b: Option<&'a str>,
        formatted_access_modifier: FormattedAccessModifier,
        class_sub_sections: Vec<ClassSubSection<'a>>,
    },
    COMMENT(&'a str),
    FIELD {
        field_name_a: &'a str,
        field_name_b: Option<&'a str>,
        formatted_access_modifier: FormattedAccessModifier,
        field_desc_a: &'a str,
        field_sub_sections: Vec<&'a str>,
    },
    METHOD {
        method_name_a: &'a str,
        method_name_b: Option<&'a str>,
        formatted_access_modifier: FormattedAccessModifier,
        method_desc_a: &'a str,
        method_sub_sections: Vec<MethodSubSections>,
    },
}
