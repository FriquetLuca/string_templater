# String Templater

> A little string templater library to write down some string templates and injecting datas in it.

## How to use

Create your object and define it's fields.

```rs
#[derive(Debug, Serialize)]
struct Person {
  pub name: String,
  pub child: Option<Person>,
}
```

The object will be used specifically for data injection, so if you have optional data, make sure it has some data.
In the `None` case, a `StringTemplaterError::UnknownField` will be returned.
For problematic curvy bracket, you'll get a `StringTemplaterError::MissingCurvyBracket` error and in the case of a serialize error, a `StringTemplaterError::SerializeError` will be returned.

Now, write down some string template and enjoy:
```rs
use string_templater::prelude::*;

fn main() {
  let someone = Person {
    name: "Roger".to_string(),
    child: Some(Person {
      name: "Betty".to_string(),
      child: Some(Person {
        name: "Kenny".to_string(),
        child: None,
      }),
    }),
  };
  // Print: Hello Roger! Is Betty your child? So your grandchild must be Kenny.
  println!(parse("Hello {{name}}! Is {{child.name}} your child? So your grandchild must be {{child.child.name}}.", &someone).unwrap());
}
```

It support `JSON` object and array for the path notation, making templating text easy.

```rs
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
let result = parse(template_str, &classroom).unwrap(); // Display : You should be alice.
```

## Mirroring datas

For some reason, it might be possible that you need to access the value of a key in the hashmap based on the value of a field. Here, we're calling this behaviour mirroring since it's similar to a laser pointing towards a mirror then being redirected.

First of, let's create a set of datas to show mirroring in action (you can also parse a structure if you want, but it'll be faster to just think of our final datas instead of the full structure).
```rs
let mut a: HashMap<String, String> = HashMap::new();
a.insert("name".to_string(), "Doe".to_string());
a.insert("age".to_string(), "35".to_string());
a.insert("key_name".to_string(), "age".to_string());
```

To mirror a value, you'll need to use the `*` symbol before the path of your variable. This will use the value of the selected key as a new key to search a value. You can also use multiple mirroring into another using multiples `*`.
You can see what it does in the following example:
```rs
let template_str = "Hello {{name}}! Is it true that you're {{*key_name}} years old?";
println!(generate(template_str, &a).unwrap()); // Display : Hello Doe! Is it true that you're 35 years old?
```

## Nested templates

There might be reasons where we would like to use multiple templates, nested inside each other. For this very reason, you'll need to write down your string interpolation using the triple curly bracket notation.

```rs
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
println!(generate(template_str, &a).unwrap()); // Display : Hello Doe! I only have 2 beers for you today... Well, bye Doe! Enjoy your 2 beers.
```

Nested templates works with mirroring so you can use a mirroring template using `{{{*my_key}}}` pattern.

Be warned: you need to be careful when using nesting with templates to avoid creating an infinite loop by searching a template in a template calling an already called template.

## Parse to HashMap

It may be possible that you need to transform your datas into a hashmap to do something on it before consuming it.
For this very reason, you can access the `parse_to_hashmap` function.

```rs
fn parse_to_hashmap<T: ?Sized + Serialize>(value: &T) -> Result<HashMap<String, String>, StringTemplaterError>;
```

The keys of the hashmap contain the parent/child field path of a value using the dot notation.
