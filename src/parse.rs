use serde::Serialize;

use crate::{errors::StringTemplaterError, generate::generate, parse_to_hashmap::parse_to_hashmap};

/// Inject values of a serializable object based on their field names.
/// Datas are injected on a specific string interpolation using the pattern `{{field_a.subfield_a}}`.
/// You can now think of your struct as an object containing fields, the same way javascript does.
/// This allow a simple navigation around the datas.
pub fn parse<T: ?Sized + Serialize>(
    template_str: &str,
    data: &T,
) -> Result<String, StringTemplaterError> {
    match parse_to_hashmap(&data) {
        Ok(datas) => generate(template_str, &datas),
        Err(err) => Err(err),
    }
}
