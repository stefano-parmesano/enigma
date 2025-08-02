use crate::error::EnigmaError;

pub struct ContentOwned {
    pub classes: Vec<ClassOwned>,
}

pub enum ClassSubSectionOwned {
    FIELD {
        field_name_a: String,
        field_name_b: Option<String>,
        field_desc_a: String,
    },
    METHOD {
        method_name_a: String,
        method_name_b: Option<String>,
        method_desc_a: String,
    },
}

pub struct ClassOwned {
    pub class_name_a: String,
    pub class_name_b: Option<String>,
    pub formatted_access_modifier: Option<String>,
    pub class_sub_sections: Vec<ClassSubSectionOwned>,
}

impl ContentOwned {
    fn from_str<'b>(s: String) -> Result<Self, EnigmaError<'b>> {
        let mut classes: Vec<ClassOwned> = Vec::new();
        let mut class_sub_section_build: Vec<ClassSubSectionOwned> = Vec::new();
        let mut lines = s.lines();
        while let Some(line) = lines.next_back() {
            let mut words = line.split_whitespace();
            match words.next() {
                Some("") => continue,
                Some("CLASS") => {
                    let class_name_a = words
                        .next()
                        .ok_or(EnigmaError("Class name a not found"))?
                        .to_owned();
                    let class_name_b = words.next().map(str::to_owned);
                    let formatted_access_modifier = words.next().map(str::to_owned);
                    let class_sub_sections = std::mem::take(&mut class_sub_section_build);
                    classes.push(ClassOwned {
                        class_name_a,
                        class_name_b,
                        formatted_access_modifier,
                        class_sub_sections,
                    });
                }
                Some(identifier) => match identifier {
                    "FIELD" => {
                        let field_name_a = words
                            .next()
                            .ok_or(EnigmaError("Field name a not found"))?
                            .to_owned();
                        let field_name_b_or_field_desc_a = words
                            .next()
                            .ok_or(EnigmaError("Field desc a not found"))?
                            .to_owned();
                        if let Some(field_desc_a) = words.next() {
                            class_sub_section_build.push(ClassSubSectionOwned::FIELD {
                                field_name_a,
                                field_name_b: Some(field_name_b_or_field_desc_a),
                                field_desc_a: field_desc_a.to_owned(),
                            });
                        } else {
                            class_sub_section_build.push(ClassSubSectionOwned::FIELD {
                                field_name_a,
                                field_name_b: None,
                                field_desc_a: field_name_b_or_field_desc_a,
                            });
                        }
                    }
                    "METHOD" => {
                        let method_name_a = words
                            .next()
                            .ok_or(EnigmaError("Field name a not found"))?
                            .to_owned();
                        let method_name_b_or_method_desc_a = words
                            .next()
                            .ok_or(EnigmaError("Field desc a not found"))?
                            .to_owned();
                        if let Some(method_desc_a) = words.next() {
                            class_sub_section_build.push(ClassSubSectionOwned::METHOD {
                                method_name_a,
                                method_name_b: Some(method_name_b_or_method_desc_a),
                                method_desc_a: method_desc_a.to_owned(),
                            });
                        } else {
                            class_sub_section_build.push(ClassSubSectionOwned::METHOD {
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

pub struct FileOwned {
    pub content: ContentOwned,
}

impl FileOwned {
    pub fn from_string<'b>(s: String) -> Result<Self, EnigmaError<'b>> {
        ContentOwned::from_str(s).map(|content| Self { content })
    }
}

pub fn from_string<'b>(s: String) -> Result<FileOwned, EnigmaError<'b>> {
    FileOwned::from_string(s)
}
