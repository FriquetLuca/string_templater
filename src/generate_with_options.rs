use crate::{errors::StringTemplaterError, StringTemplaterOptions};
use std::collections::HashMap;

/// Inject values of a hashmap based on their field names.
/// Datas are injected on a specific string interpolation using the pattern `{{key_name}}`.
/// This allow a simple navigation around the datas.
pub fn generate_with_options(
    template_str: &str,
    data: &HashMap<String, String>,
    option: &StringTemplaterOptions,
) -> Result<String, StringTemplaterError> {
    let mut result = String::new();
    let mut chars = template_str.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                if let Some('{') = chars.peek() {
                    chars.next();

                    let mut apply_template = false;
                    let mut mirror = false;
                    let mut mirror_count = 0;
                    let mut key = String::new();

                    // Templating
                    if let Some('{') = chars.peek() {
                        apply_template = true;
                        chars.next();
                    }

                    // Mirroring
                    if let Some('*') = chars.peek() {
                        mirror = true;
                        chars.next();
                        mirror_count = 1;
                        while let Some('*') = chars.peek() {
                            chars.next();
                            mirror_count += 1;
                        }
                    }

                    // Key looking
                    while let Some(&next) = chars.peek() {
                        if next == '}' {
                            break;
                        } else if next == '\\' {
                            chars.next();
                            match chars.peek() {
                                Some('*') => {
                                    chars.next();
                                    key.push('*');
                                }
                                Some('\\') => {
                                    chars.next();
                                    key.push('\\');
                                }
                                Some('{') => {
                                    chars.next();
                                    key.push('{');
                                }
                                Some('}') => {
                                    chars.next();
                                    key.push('}');
                                }
                                _ => key.push('\\'),
                            }
                        } else {
                            key.push(chars.next().unwrap());
                        }
                    }

                    // Error handling
                    if let Some('}') = chars.peek() {
                        chars.next();
                    } else if apply_template {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing three curvy bracket `}}` around `{}`.",
                            key
                        )));
                    } else {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing two curvy bracket `}}` around `{}`.",
                            key
                        )));
                    }
                    if let Some('}') = chars.peek() {
                        chars.next();
                    } else if apply_template {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing two curvy bracket `}}` around `{}`.",
                            key
                        )));
                    } else {
                        return Err(StringTemplaterError::MissingCurvyBracket(format!(
                            "Missing one curvy bracket `}}` around `{}`.",
                            key
                        )));
                    }
                    if apply_template {
                        if let Some('}') = chars.peek() {
                            chars.next();
                        } else {
                            return Err(StringTemplaterError::MissingCurvyBracket(format!(
                                "Missing one curvy bracket `}}` around `{}`.",
                                key
                            )));
                        }
                    }

                    // Data handling
                    if let Some(value) = data.get(&key) {
                        let data_value = if mirror {
                            // Apply C mirrors
                            let mut maybe_mirror = data.get(value);
                            mirror_count -= 1;
                            while mirror_count > 0 {
                                if let Some(value) = maybe_mirror {
                                    maybe_mirror = data.get(value);
                                } else if option.safe_parse {
                                    if option.display_missing_keys {
                                        let patched_value = option.override_missing_keys.as_ref();
                                        let patched_value = patched_value
                                            .map(|f| f(value))
                                            .unwrap_or(format!("[MISSING_KEY: `{}`]", value));
                                        result.push_str(&patched_value);
                                    }
                                } else {
                                    return Err(StringTemplaterError::UnknownField(format!(
                                        "The field `{}` does not exist in the hashmap.",
                                        value
                                    )));
                                }
                                mirror_count -= 1;
                            }
                            maybe_mirror
                        } else {
                            Some(value)
                        };

                        if let Some(value) = data_value {
                            if apply_template {
                                let generated = generate_with_options(value.as_str(), data, option);
                                if let Err(err) = generated {
                                    return Err(err);
                                } else if let Ok(value) = generated {
                                    result.push_str(&value);
                                }
                            } else {
                                result.push_str(value);
                            }
                        } else if option.safe_parse {
                            if option.display_missing_keys {
                                let patched_value = option.override_missing_keys.as_ref();
                                let patched_value = patched_value
                                    .map(|f| f(value))
                                    .unwrap_or(format!("[MISSING_KEY: `{}`]", value));
                                result.push_str(&patched_value);
                            }
                        } else {
                            return Err(StringTemplaterError::UnknownField(format!(
                                "The field `{}` does not exist in the hashmap.",
                                value
                            )));
                        }
                    } else if option.safe_parse {
                        if option.display_missing_keys {
                            let patched_value = option.override_missing_keys.as_ref();
                            let patched_value = patched_value
                                .map(|f| f(&key))
                                .unwrap_or(format!("[MISSING_KEY: `{}`]", key));
                            result.push_str(&patched_value);
                        }
                    } else {
                        return Err(StringTemplaterError::UnknownField(format!(
                            "The field `{}` does not exist in the hashmap.",
                            key
                        )));
                    }
                } else {
                    result.push('{');
                }
            }
            '\\' => {
                if let Some(&next) = chars.peek() {
                    if next == '{' {
                        chars.next();
                        result.push('{');
                    } else if next == '}' {
                        chars.next();
                        result.push('}');
                    } else if next == '\\' {
                        chars.next();
                        result.push(c);
                    } else {
                        result.push(c);
                    }
                } else {
                    result.push(c);
                }
            }
            _ => result.push(c),
        };
    }
    Ok(result)
}
