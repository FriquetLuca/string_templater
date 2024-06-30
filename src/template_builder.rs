use serde::Serialize;
use std::collections::HashMap;

use crate::{
    generate_with_options, parse_to_hashmap, OverrideMessage, StringTemplaterError,
    StringTemplaterOptions,
};

pub struct TemplateBuilder {
    template_values: HashMap<String, String>,
    options: StringTemplaterOptions,
}

impl Default for TemplateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateBuilder {
    pub fn new() -> Self {
        TemplateBuilder {
            template_values: HashMap::new(),
            options: StringTemplaterOptions {
                safe_parse: false,
                display_missing_keys: false,
                override_missing_keys: None,
            },
        }
    }
    /// Insert a key / value for the template to use.
    pub fn insert(&mut self, key: &str, value: &str) {
        self.template_values.insert(key.into(), value.into());
    }
    /// Insert a collection of key / value for the template to use.
    pub fn insert_hashmap(&mut self, data: &HashMap<String, String>) {
        for (key, value) in data {
            self.insert(key, value);
        }
    }
    /// Insert a collection of key / value from the struct for the template to use.
    /// This method can fail while parsing and let you handle the error.
    pub fn insert_struct<T: ?Sized + Serialize>(
        &mut self,
        data: &T,
    ) -> Result<(), StringTemplaterError> {
        match parse_to_hashmap(&data) {
            Ok(datas) => {
                self.insert_hashmap(&datas);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
    /// Display the missing key fields.
    pub fn display_missing_keys(&mut self) {
        self.options = StringTemplaterOptions {
            safe_parse: self.options.safe_parse,
            display_missing_keys: true,
            override_missing_keys: self.options.override_missing_keys.take(),
        }
    }
    /// Hide missing key fields.
    pub fn hide_missing_keys(&mut self) {
        self.options = StringTemplaterOptions {
            safe_parse: self.options.safe_parse,
            display_missing_keys: false,
            override_missing_keys: self.options.override_missing_keys.take(),
        }
    }
    /// Override the missing key fields using a specific method.
    pub fn override_missing_keys_message(&mut self, override_missing_keys: OverrideMessage) {
        self.options = StringTemplaterOptions {
            safe_parse: self.options.safe_parse,
            display_missing_keys: self.options.display_missing_keys,
            override_missing_keys: Some(override_missing_keys),
        }
    }
    /// Safe parse allow you to ignore errors comming from missing keys.
    pub fn set_safe_parse(&mut self, safe_parse: bool) {
        self.options = StringTemplaterOptions {
            safe_parse,
            display_missing_keys: self.options.display_missing_keys,
            override_missing_keys: self.options.override_missing_keys.take(),
        }
    }
    /// Insert a collection of key / value from the struct for the template to use.
    /// This method either insert datas if it succeed or do nothing.
    pub fn insert_safe_struct<T: ?Sized + Serialize>(&mut self, data: &T) {
        if let Ok(datas) = parse_to_hashmap(&data) {
            self.insert_hashmap(&datas);
        }
    }
    /// Build the chosen template using the values of the builder.
    pub fn build(&self, template_str: &str) -> Result<String, StringTemplaterError> {
        generate_with_options(template_str, &self.template_values, &self.options)
    }
}
