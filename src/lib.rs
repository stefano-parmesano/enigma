mod error;
pub mod owned;

use crate::error::EnigmaError;

pub struct Content<'a> {
    pub classes: Vec<Class<'a>>,
}

pub enum ClassSubSection<'a> {
    FIELD {
        field_name_a: &'a str,
        field_name_b: Option<&'a str>,
        field_desc_a: &'a str,
    },
    METHOD {
        method_name_a: &'a str,
        method_name_b: Option<&'a str>,
        method_desc_a: &'a str,
    },
}

pub struct Class<'a> {
    pub class_name_a: &'a str,
    pub class_name_b: Option<&'a str>,
    pub formatted_access_modifier: Option<&'a str>,
    pub class_sub_sections: Vec<ClassSubSection<'a>>,
}

impl<'a> Content<'a> {
    fn from_str<'b>(s: &'a str) -> Result<Self, EnigmaError<'b>> {
        let mut classes: Vec<Class> = Vec::new();
        let mut class_sub_section_build: Vec<ClassSubSection> = Vec::new();
        let mut lines = s.lines();
        while let Some(line) = lines.next_back() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("") => continue,
                Some("CLASS") => {
                    let class_name_a = words.next().ok_or(EnigmaError("Class name a not found"))?;
                    let class_name_b = words.next();
                    let formatted_access_modifier = words.next();
                    let class_sub_sections = std::mem::take(&mut class_sub_section_build);
                    classes.push(Class {
                        class_name_a,
                        class_name_b,
                        formatted_access_modifier,
                        class_sub_sections,
                    });
                }
                Some(identifier) => match identifier {
                    "FIELD" => {
                        let field_name_a =
                            words.next().ok_or(EnigmaError("Field name a not found"))?;
                        let field_name_b_or_field_desc_a =
                            words.next().ok_or(EnigmaError("Field desc a not found"))?;
                        if let Some(field_desc_a) = words.next() {
                            class_sub_section_build.push(ClassSubSection::FIELD {
                                field_name_a,
                                field_name_b: Some(field_name_b_or_field_desc_a),
                                field_desc_a,
                            });
                        } else {
                            class_sub_section_build.push(ClassSubSection::FIELD {
                                field_name_a,
                                field_name_b: None,
                                field_desc_a: field_name_b_or_field_desc_a,
                            });
                        }
                    }
                    "METHOD" => {
                        let method_name_a =
                            words.next().ok_or(EnigmaError("Field name a not found"))?;
                        let method_name_b_or_method_desc_a =
                            words.next().ok_or(EnigmaError("Field desc a not found"))?;
                        if let Some(method_desc_a) = words.next() {
                            class_sub_section_build.push(ClassSubSection::METHOD {
                                method_name_a,
                                method_name_b: Some(method_name_b_or_method_desc_a),
                                method_desc_a,
                            });
                        } else {
                            class_sub_section_build.push(ClassSubSection::METHOD {
                                method_name_a,
                                method_name_b: None,
                                method_desc_a: method_name_b_or_method_desc_a,
                            });
                        }
                    }
                    "CLASS" | "COMMENT" | "ARG" => continue,
                    _ => return Err(EnigmaError("Invalid Identifier")),
                },
                None => return Err(EnigmaError("Line empty")),
            }
        }
        Ok(Self { classes })
    }
}

pub struct File<'a> {
    pub content: Content<'a>,
}

impl<'a> File<'a> {
    pub fn from_str<'b>(s: &'a str) -> Result<Self, EnigmaError<'b>> {
        Content::from_str(s).map(|content| Self { content })
    }
}

pub fn from_str<'b>(s: &str) -> Result<File<'_>, EnigmaError<'b>> {
    File::from_str(s)
}
