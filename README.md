# builder-pattern

A derivable macro for declaring a builder pattern.

## Installation

```toml
[dependencies]
builder-pattern = "0.1"
```

## Examples

```rust
use builder_pattern::*;

#[derive(Builder)]
struct Person {
    name: String,
    age: i32,
    #[default(Gender::Nonbinary)]
    gender: Gender,
}

let p1 = Person::new().name(String::from("Joe")).age(27).build();
// Orders does not matter.
let p2 = Person::new().age(32).name(String::from("Jack")).gender(Gender::Male).build();
// `name` field required - Compilation error.
let p3 = Person::new().age(15).build();
```

It considers all fields without `default` attribute as required.
If the attribute is provided, the expression in the parantheses is evaluated as a default value.

When the insuficient number of arguments is provided, the compilation will fail.
