//! case conversion utilities

use std::borrow::Cow;

#[allow(dead_code)]
pub enum Case {
    /// UpperCamelCase
    UpperCamel,
    /// PascalCase
    Pascal,
    /// snake_case
    Snake,
}

pub fn convert(input: &str, case: Case) -> Cow<'_, str> {
    let converted: String = match case {
        Case::UpperCamel | Case::Pascal => heck::AsPascalCase(input).to_string(),
        Case::Snake => heck::AsSnakeCase(input).to_string(),
    };

    if converted == input {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(converted)
    }
}
