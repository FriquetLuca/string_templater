use serde::Serialize;

use crate::{
    errors::StringTemplaterError, generate_with_options::generate_with_options,
    parse_to_hashmap::parse_to_hashmap, StringTemplaterOptions,
};

/// Inject values of a serializable object based on their field names.
/// Datas are injected on a specific string interpolation using the pattern `{{field_a.subfield_a}}`.
/// You can now think of your struct as an object containing fields, the same way javascript does.
/// This allow a simple navigation around the datas.
pub fn parse_with_options<T: ?Sized + Serialize>(
    template_str: &str,
    data: &T,
    option: &StringTemplaterOptions,
) -> Result<String, StringTemplaterError> {
    match parse_to_hashmap(&data) {
        Ok(datas) => generate_with_options(template_str, &datas, option),
        Err(err) => Err(err),
    }
}
