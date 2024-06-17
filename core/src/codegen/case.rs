use std::borrow::Cow;

#[allow(dead_code)]
pub enum Case {
    /// camelCase
    Camel,
    /// UpperCamelCase
    UpperCamel,
    /// PascalCase
    Pascal,
    /// snake_case
    Snake,
    /// SCREAMING_SNAKE_CASE
    ScreamingSnake,
    /// kebab-case
    Kebab,
}

pub fn convert(input: &str, case: Case) -> Cow<'_, str> {
    let converted: String = match case {
        Case::Camel => heck::AsLowerCamelCase(input).to_string(),
        Case::UpperCamel | Case::Pascal => heck::AsPascalCase(input).to_string(),
        Case::Snake => heck::AsSnakeCase(input).to_string(),
        Case::ScreamingSnake => heck::AsShoutySnakeCase(input).to_string(),
        Case::Kebab => heck::AsKebabCase(input).to_string(),
    };

    if converted == input {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(converted)
    }
}
