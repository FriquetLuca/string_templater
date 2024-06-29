use crate::{errors::StringTemplaterError, generate_with_options, StringTemplaterOptions};
use std::collections::HashMap;

/// Inject values of a hashmap based on their field names.
/// Datas are injected on a specific string interpolation using the pattern `{{key_name}}`.
/// This allow a simple navigation around the datas.
pub fn generate(
    template_str: &str,
    data: &HashMap<String, String>,
) -> Result<String, StringTemplaterError> {
    generate_with_options(template_str, data, &StringTemplaterOptions::default())
}
