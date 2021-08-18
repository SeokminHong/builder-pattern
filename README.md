# builder-pattern

A derivable macro for declaring a builder pattern.

## Usage

```rust
use builder_pattern::Builder;

#[derive(Builder)]
struct Person {
    #[setter(into)]
    name: String,
    age: i32,
    #[default(Gender::Nonbinary)]
    gender: Gender,
}

let p1 = Person::new()
    .name(String::from("Joe"))
    .age(27)
    .build();

// Orders does not matter.
let p2 = Person::new()
    .age(32)
    // `&str` is implicitly converted into `String`
    // because of `setter(into)` attribute!
    .name("Jack")
    .gender(Gender::Male).build();

// `name` field required - Compilation error.
let p3 = Person::new()
    .age(15)
    .build();
```

## Get Started

Add `builder-pattern` to `Cargo.toml`.

```toml
// Cargo.toml
[dependencies]
builder-pattern = "0.3"
```

## Features

- **Chaining**: Can make structure with chained setters.
- **Complex types are supported**: Lifetime and trait bounds, and where clauses are well supported.
- **Type safety**: Autocompletion tools can suggest correct setters to build the struct. Also, `build` function is allowed only the all of required fields are provided. **No Result**, **No Unwrap**. Just use it.

## Attributes

WIP

### default

### setter

### validator
