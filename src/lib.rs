mod errors;
mod generate;
mod generate_with_options;
mod options;
mod parse;
mod parse_to_hashmap;
mod parse_with_options;

pub use crate::errors::StringTemplaterError;
pub use crate::generate::generate;
pub use crate::generate_with_options::generate_with_options;
pub use crate::options::{OverrideMessage, StringTemplaterOptions};
pub use crate::parse::parse;
pub use crate::parse_to_hashmap::parse_to_hashmap;
pub use crate::parse_with_options::parse_with_options;

#[cfg(test)]
mod test {
    use crate::*;
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Debug, Serialize)]
    struct A {
        pub name: String,
        pub son: B,
    }

    #[derive(Debug, Serialize)]
    struct B {
        pub name: String,
        pub age: i32,
    }

    #[derive(Debug, Serialize)]
    struct M {
        pub name: String,
        pub age: i32,
        pub key_name: String,
    }

    #[derive(Debug, Serialize)]
    struct V {
        pub items: Vec<W>,
    }

    #[derive(Debug, Serialize)]
    struct W {
        pub name: String,
    }

    #[derive(Debug, Serialize)]
    struct Person {
        pub name: String,
        pub child: Option<Box<Person>>,
    }

    #[test]
    fn test_write_field() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("surname".to_string(), "Doey".to_string());

        let template_str = "Hello {{name}}! Or should I call you {{surname}}?";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(result, "Hello Doe! Or should I call you Doey?".to_string())
    }

    #[test]
    fn test_escape_write_field() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());

        let template_str = "Hello \\{{name}}!";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(result, "Hello {{name}}!".to_string())
    }

    #[test]
    fn test_escape_write_field_2() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());

        let template_str = "Hello {\\{name}}!";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(result, "Hello {{name}}!".to_string())
    }

    #[test]
    fn test_antislash_write_field() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());

        let template_str = "Hello \\\\{{name}}!";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(result, "Hello \\Doe!".to_string())
    }

    #[test]
    fn test_write_field_failed_on_missing_curvy_bracket_1() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("surname".to_string(), "Doey".to_string());

        let template_str = "Hello {{name}!";
        let result = match generate(template_str, &a) {
            Ok(_) => StringTemplaterError::SerializeError("".to_string()),
            Err(err) => err,
        };
        assert_eq!(
            result,
            StringTemplaterError::MissingCurvyBracket(
                "Missing one curvy bracket `}` around `name`.".to_string()
            )
        )
    }

    #[test]
    fn test_write_field_failed_on_missing_curvy_bracket_2() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("surname".to_string(), "Doey".to_string());

        let template_str = "Hello {{name! How're you today?";
        let result = match generate(template_str, &a) {
            Ok(_) => StringTemplaterError::SerializeError("".to_string()),
            Err(err) => err,
        };
        assert_eq!(
            result,
            StringTemplaterError::MissingCurvyBracket(
                "Missing two curvy bracket `}` around `name! How're you today?`.".to_string()
            )
        )
    }

    #[test]
    fn test_write_field_failed_on_unknown_field() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("surname".to_string(), "Doey".to_string());

        let template_str = "Hello {{namey}}!";
        let result = match generate(template_str, &a) {
            Ok(_) => StringTemplaterError::SerializeError("".to_string()),
            Err(err) => err,
        };
        assert_eq!(
            result,
            StringTemplaterError::UnknownField(
                "The field `namey` does not exist in the hashmap.".to_string()
            )
        )
    }

    #[test]
    fn test_serialize_and_write_field() {
        let a = A {
            name: "John".to_string(),
            son: B {
                name: "Doe".to_string(),
                age: 20,
            },
        };
        let template_str = "Hello {{name}}! Your son's name is {{son.name}} and he is \\\\{{son.age}} years old. Welcome to the \\{Rust\\} world.";
        let result = parse(template_str, &a).unwrap();
        assert_eq!(result, "Hello John! Your son's name is Doe and he is \\20 years old. Welcome to the {Rust} world.".to_string())
    }

    #[test]
    fn test_serialize_and_write_nested_field() {
        let a = Person {
            name: "Roger".to_string(),
            child: Some(Box::new(Person {
                name: "Betty".to_string(),
                child: Some(Box::new(Person {
                    name: "Kenny".to_string(),
                    child: None,
                })),
            })),
        };
        let template_str = "Hello {{name}}! Is {{child.name}} your child? So your grandchild must be {{child.child.name}}.";
        let result = parse(template_str, &a).unwrap();
        assert_eq!(
            result,
            "Hello Roger! Is Betty your child? So your grandchild must be Kenny.".to_string()
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_failed_on_none() {
        let a = Person {
            name: "Roger".to_string(),
            child: Some(Box::new(Person {
                name: "Betty".to_string(),
                child: None,
            })),
        };
        let template_str = "Hello {{name}}! Is {{child.name}} your child? So your grandchild must be {{child.child.name}}.";
        let result = match parse(template_str, &a) {
            Ok(_) => StringTemplaterError::MissingCurvyBracket("".to_string()),
            Err(err) => err,
        };
        assert_eq!(
            result,
            StringTemplaterError::UnknownField(
                "The field `child.child.name` does not exist in the hashmap.".to_string()
            )
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_mirror() {
        let a = M {
            name: "Doe".to_string(),
            age: 35,
            key_name: "age".to_string(),
        };
        let template_str = "Hello {{name}}! Is it true that you're {{*key_name}} years old?";
        let result = parse(template_str, &a).unwrap();
        assert_eq!(
            result,
            "Hello Doe! Is it true that you're 35 years old?".to_string()
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_multi_mirrors() {
        let a = M {
            name: "key_name".to_string(),
            age: 35,
            key_name: "age".to_string(),
        };
        let template_str = "Your key is named {{name}} and gives of the value {{**name}}.";
        let result = parse(template_str, &a).unwrap();
        assert_eq!(
            result,
            "Your key is named key_name and gives of the value 35.".to_string()
        )
    }

    #[test]
    fn test_escape_mirror() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("age".to_string(), "35".to_string());
        a.insert("*key_name".to_string(), "age".to_string());

        let template_str = "Hello {{name}}! What's your {{\\*key_name}}?";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(result, "Hello Doe! What's your age?".to_string())
    }

    #[test]
    fn test_nested_templates() {
        let mut a: HashMap<String, String> = HashMap::new();
        a.insert("time".to_string(), "today".to_string());
        a.insert("name".to_string(), "Doe".to_string());
        a.insert("beers_count".to_string(), "2".to_string());
        a.insert(
            "dialog".to_string(),
            "I only have {{beers_count}} beers for you {{time}}".to_string(),
        );
        a.insert(
            "bye".to_string(),
            "Well, bye {{name}}! {{{polite}}}".to_string(),
        );
        a.insert(
            "polite".to_string(),
            "Enjoy your {{beers_count}} beers.".to_string(),
        );

        let template_str = "Hello {{name}}! {{{dialog}}}... {{{bye}}}";
        let result = generate(template_str, &a).unwrap();
        assert_eq!(
            result,
            "Hello Doe! I only have 2 beers for you today... Well, bye Doe! Enjoy your 2 beers."
                .to_string()
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_mirror_can_fail() {
        let a = M {
            name: "Doe".to_string(),
            age: 35,
            key_name: "surname".to_string(),
        };
        let template_str = "Hello {{name}}! Is it true that you're {{*key_name}} years old?";
        let result = match parse(template_str, &a) {
            Ok(_) => StringTemplaterError::MissingCurvyBracket("".to_string()),
            Err(err) => err,
        };
        assert_eq!(
            result,
            StringTemplaterError::UnknownField(
                "The field `surname` does not exist in the hashmap.".to_string()
            )
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_mirror_can_fail_with_options_display_missing_fields() {
        let a = M {
            name: "Doe".to_string(),
            age: 35,
            key_name: "surname".to_string(),
        };
        let template_str = "Hello {{name}}! Is it true that you're {{current_age}} years old?";
        let result = parse_with_options(
            template_str,
            &a,
            &StringTemplaterOptions {
                safe_parse: true,
                display_missing_keys: true,
                override_missing_keys: None,
            },
        )
        .unwrap();
        assert_eq!(
            result,
            "Hello Doe! Is it true that you're [MISSING_KEY: `current_age`] years old?".to_string(),
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_mirror_can_fail_with_options_display_missing_fields_override(
    ) {
        let a = M {
            name: "Doe".to_string(),
            age: 35,
            key_name: "surname".to_string(),
        };
        let template_str = "Hello {{name}}! Is it true that you're {{current_age}} years old?";
        let result = parse_with_options(
            template_str,
            &a,
            &StringTemplaterOptions {
                safe_parse: true,
                display_missing_keys: true,
                override_missing_keys: Some(Box::new(move |s| format!("[key `{}` is missing]", s))),
            },
        )
        .unwrap();
        assert_eq!(
            result,
            "Hello Doe! Is it true that you're [key `current_age` is missing] years old?"
                .to_string(),
        )
    }

    #[test]
    fn test_serialize_and_write_nested_field_mirror_can_fail_with_options_hide_missing_fields() {
        let a = M {
            name: "Doe".to_string(),
            age: 35,
            key_name: "surname".to_string(),
        };
        let template_str = "Hello {{name}}! Is it true that you're {{current_age}} years old?";
        let result = parse_with_options(
            template_str,
            &a,
            &StringTemplaterOptions {
                safe_parse: true,
                display_missing_keys: false,
                override_missing_keys: None,
            },
        )
        .unwrap();
        assert_eq!(
            result,
            "Hello Doe! Is it true that you're  years old?".to_string(),
        )
    }

    #[test]
    fn test_serialize_vector_field_and_index() {
        let a = V {
            items: vec![
                W {
                    name: "john".to_string(),
                },
                W {
                    name: "janne".to_string(),
                },
                W {
                    name: "alice".to_string(),
                },
                W {
                    name: "bob".to_string(),
                },
            ],
        };
        let template_str = "You should be {{items.2.name}}.";
        let result = parse(template_str, &a).unwrap();
        assert_eq!(result, "You should be alice.".to_string())
    }
}
