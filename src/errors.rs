/// Any errors that might occurs while generating or parsing the string template.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum StringTemplaterError {
    #[error("Missing curvy bracket: `{0}`")]
    MissingCurvyBracket(String),
    #[error("Unknown field: `{0}`")]
    UnknownField(String),
    #[error("Serialize error: `{0}`")]
    SerializeError(String),
}
