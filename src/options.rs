pub type OverrideMessage = Box<dyn Fn(&String) -> String>;

#[derive(Default)]
pub struct StringTemplaterOptions {
    pub safe_parse: bool,
    pub display_missing_keys: bool,
    pub override_missing_keys: Option<OverrideMessage>,
}
