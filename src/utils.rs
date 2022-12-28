use convert_case::Case;

pub fn str_to_case(s: &str) -> Case {
    match s {
        "snake" => Case::Snake,
        "kebab" => Case::Kebab,
        "camel" => Case::Camel,
        "pascal" => Case::Pascal,
        "upper" => Case::Upper,
        "lower" => Case::Lower,
        "title" => Case::Title,
        "toggle" => Case::Toggle,
        "upper_camel" => Case::UpperCamel,
        "upper_snake" => Case::UpperSnake,
        "cobol" => Case::Cobol,
        "upper_kebab" => Case::UpperKebab,
        "train" => Case::Train,
        "flat" => Case::Flat,
        "upper_flat" => Case::UpperFlat,
        "alternating" => Case::Alternating,
        _ => Case::Snake,
    }
}
