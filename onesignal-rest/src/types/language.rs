// https://documentation.onesignal.com/docs/language-localization#what-languages-are-supported

use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

//
#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(other)]
    Other(Box<str>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser() {
        #[derive(serde::Serialize)]
        struct Foo {
            language: Language,
        }

        assert_eq!(
            serde_json::to_string(&Foo {
                language: Language::En
            })
            .unwrap(),
            r#"{"language":"en"}"#
        );
        assert_eq!(
            serde_json::to_string(&Foo {
                language: Language::ZhHans
            })
            .unwrap(),
            r#"{"language":"zh-Hans"}"#
        );
        assert_eq!(
            serde_json::to_string(&Foo {
                language: Language::Other("vi".into())
            })
            .unwrap(),
            r#"{"language":"vi"}"#
        );
    }
}
