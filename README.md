# String Templater

> A little string templater library to write down some string templates and injecting datas in it.

## Syntax

The syntax is easy and requires you to know only some rules.
1. The `\` symbol followed by `{`, `}` or `\` will always escape the next character, making `\` ignored in the output.
2. To access the value of a key, you'll use the notation `{{my_key_name}}`.
3. The `*` symbol (mirror operator) will only work right after `{{` and before the key name.
4. When parsing the key name, the symbol `\` followed by `*` will result in the character `*` being outputed.
5. Escaping the `*` symbol is only useful right after a `{{`.
6. The mirror operator can be applied as many times as it exist on the key name.
7. To use the value of a key as a template, you'll use the notation `{{{my_key_name}}}`.
8. You can have as many template as you want inside other template, but be careful of infinite loops.
9. The path for key names when serializing datas look as close as `JS` accessors as possible (with the exception of array).
10. To access an array index, use the dot notation instead of the bracket notation from `JS` (ex: `my_field.2.name`).

## How to use

Create your object and define it's fields.

```rs
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Person {
  pub name: String,
  pub child: Option<Box<Person>>,
}
```

The object will be used specifically for data injection, so if you have optional data, make sure it has some data.
In the `None` case, a `StringTemplaterError::UnknownField` will be returned.
For problematic curvy bracket, you'll get a `StringTemplaterError::MissingCurvyBracket` error and in the case of a serialize error, a `StringTemplaterError::SerializeError` will be returned.

Now, write down some string template and enjoy:
```rs
use string_templater::parse;

fn main() {
  let someone = Person {
    name: "Roger".to_string(),
    child: Some(Box::new(Person {
      name: "Betty".to_string(),
      child: Some(Box::new(Person {
        name: "Kenny".to_string(),
        child: None,
      })),
    })),
  };
  let result = parse("Hello {{name}}! Is {{child.name}} your child? So your grandchild must be {{child.child.name}}.", &someone).unwrap();
  println!("{}", result); // Display : Hello Roger! Is Betty your child? So your grandchild must be Kenny.
}
```

It support `JSON` object and array for the path notation, making templating text easy.

```rs
use serde::Serialize;
use string_templater::parse;

...

#[derive(Debug, Serialize)]
struct Classroom {
  pub students: Vec<Child>,
}

#[derive(Debug, Serialize)]
struct Child {
  pub name: String,
}

...

let classroom = Classroom {
  students: vec![
    Child {
      name: "john".to_string(),
    },
    Child {
      name: "janne".to_string(),
    },
    Child {
      name: "alice".to_string(),
    },
    Child {
      name: "bob".to_string(),
    },
  ]
};
let template_str = "You should be {{students.2.name}}.";
println!("{}", parse(template_str, &classroom).unwrap()); // Display : You should be alice.
```

## Mirroring datas

For some reason, it might be possible that you need to access the value of a key in the hashmap based on the value of a field. Here, we're calling this behaviour mirroring since it's similar to a laser pointing towards a mirror then being redirected.

First of, let's create a set of datas to show mirroring in action (you can also parse a structure if you want, but it'll be faster to just think of our final datas instead of the full structure).
```rs
use std::collections::HashMap;
use string_templater::generate;

...

let mut data: HashMap<String, String> = HashMap::new();
data.insert("name".to_string(), "Doe".to_string());
data.insert("age".to_string(), "35".to_string());
data.insert("key_name".to_string(), "age".to_string());
```

To mirror a value, you'll need to use the `*` symbol before the path of your variable. This will use the value of the selected key as a new key to search a value. You can also use multiple mirroring into another using multiples `*`.
You can see what it does in the following example:
```rs
let template_str = "Hello {{name}}! Is it true that you're {{*key_name}} years old?";
println!("{}", generate(template_str, &data).unwrap()); // Display : Hello Doe! Is it true that you're 35 years old?
```

## Nested templates

There might be reasons where we would like to use multiple templates, nested inside each other. For this very reason, you'll need to write down your string interpolation using the triple curly bracket notation.

```rs
use std::collections::HashMap;
use string_templater::generate;

...

let mut data: HashMap<String, String> = HashMap::new();
data.insert("time".to_string(), "today".to_string());
data.insert("name".to_string(), "Doe".to_string());
data.insert("beers_count".to_string(), "2".to_string());
data.insert(
  "dialog".to_string(),
  "I only have {{beers_count}} beers for you {{time}}".to_string(),
);
data.insert(
  "bye".to_string(),
  "Well, bye {{name}}! {{{polite}}}".to_string(),
);
data.insert(
  "polite".to_string(),
  "Enjoy your {{beers_count}} beers.".to_string(),
);

let template_str = "Hello {{name}}! {{{dialog}}}... {{{bye}}}";
println!("{}", generate(template_str, &data).unwrap()); // Display : Hello Doe! I only have 2 beers for you today... Well, bye Doe! Enjoy your 2 beers.
```

Nested templates works with mirroring so you can use a mirroring template using `{{{*my_key}}}` pattern.

Be warned: you need to be careful when using nesting with templates to avoid creating an infinite loop by searching a template in a template calling an already called template.

## Options

You can use `generate_with_options` and `parse_with_options` if you want to modify the way the template generation behaves.
By default `generate` and `parse` have `safe_parse` set to `false`, so to use options you must enable this boolean.
`StringTemplaterOptions` also contains the fields `display_missing_keys` and `override_missing_keys`, thoses are used when `safe_parse` in `StringTemplaterOptions` is set to `true`.

You can now choose to display the missing keys, and if you do, you can override the default display using the `override_missing_keys` with some custom text generated by your function.

```rs
use std::collections::HashMap;
use string_templater::{generate_with_options, StringTemplaterOptions};

...

let mut data: HashMap<String, String> = HashMap::new();
data.insert("name".to_string(), "Doe".to_string());
data.insert("age".to_string(), "35".to_string());

let template_str = "Hello {{name}}! Is it true that you're {{current_age}} years old?";
println!(
  "{}",
  generate_with_options(
    template_str,
    &data,
    &StringTemplaterOptions {
      safe_parse: true,
      display_missing_keys: false,
      override_missing_keys: None,
    },
  )
  .unwrap()
); // Display : Hello Doe! Is it true that you're  years old?

println!(
  "{}",
  generate_with_options(
    template_str,
    &data,
    &StringTemplaterOptions {
      safe_parse: true,
      display_missing_keys: true,
      override_missing_keys: Some(Box::new(move |s| format!("[key `{}` is missing]", s))),
    },
  )
  .unwrap()
); // Display : Hello Doe! Is it true that you're [key `current_age` is missing] years old?
```

## Parse to HashMap

It may be possible that you need to transform your datas into a hashmap to do something on it before consuming it.
For this very reason, you can access the `parse_to_hashmap` function.

```rs
fn parse_to_hashmap<T: ?Sized + Serialize>(value: &T) -> Result<HashMap<String, String>, StringTemplaterError>;
```

The keys of the hashmap contain the parent/child field path of a value using the dot notation.
