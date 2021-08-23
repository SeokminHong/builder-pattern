# builder-pattern

[![creates.io](https://img.shields.io/crates/v/builder-pattern?logo=rust)](https://crates.io/crates/builder-pattern)
[![API Docs](https://docs.rs/builder-pattern/badge.svg?logo=docs-rs)](https://docs.rs/builder-pattern/)

A derivable macro for declaring a builder pattern.

## Usage

```rust
use builder_pattern::Builder;

enum Gender {
    Male,
    Female,
    Nonbinary
}

#[derive(Builder)]
struct Person {
    #[into]
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
    // because of `into` attribute!
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
- **No additional tasks**: There's no additional constraints to use the macro. Any structures and fields are allowed.

## Attributes

### `#[default(expr)]`

A field having this attribute will be considered as optional and the `expr` will be evaluated as a default value of the field. `build` function can be called without providing this field.

### `#[into]`

A setter function for a field having this attribute will accept an `Into` trait as a parameter. You can use this setter with implicit conversion.

Example:

```rust
#[derive(Builder)]
struct Test {
    #[into]
    pub name: String,
}

let test = Test::new()
    // `&str` is implicitly converted into `String`.
    .name("Hello")
    .build();
```

### `#[validator(expr)]`

Implement a validator for a field. `expr` could be a validating function that takes the field's type and returns `Result`.

```rust
#[derive(Builder)]
struct Test {
    #[validator(is_not_empty)]
    #[into]
    pub name: String,
}

fn is_not_empty(name: String) -> Result<String, ()> {
    if name.is_empty() {
        Err(())
    } else {
        Ok(name)
    }
}

let test1 = Test::new().name(""); // Err(())
let test2 = Test::new().name("Hello").unwrap().build();
```

## How it works

The following code

```rust
#[derive(Builder)]
struct Person {
    #[into]
    #[validator(is_not_empty)]
    name: String,
    age: i32,
    #[default(Gender::Nonbinary)]
    gender: Gender,
}

```

will generates:

```rust
struct PersonBuilder<T1, T2, T3> {
    name: Option<String>,
    age: Option<i32>,
    gender: Option<Gender>,
    _phantom: PhantomData<(T1, T2, T3)>
}

impl Person {
    // Create an empty builder
    fn new() -> PersonBuilder<(), (), ()> {
        PersonBuilder {
            name: None,
            age: None,
            // Default value
            gender: Some(Gender::Nonbinary),
            _phantom: PhantomData
        }
    }
}

// Builder for `name`.
impl<T2, T3> PersonBuilder<(), T2, T3> {
    // Receives `Into` traits.
    fn name<IntoType: Into<String>>(self, value: IntoType) ->
        Result<PersonBuilder<String, T2, T3>, ()> {
        // Validation check.
        match is_not_empty(value.into()) {
            Ok(value) => Ok(PersonBuilder {
                // Converts `IntoType` into `String`.
                name: Some(value.into()),
                age: self.age,
                gender: self.gender,
                _phantom: PhantomData,
            }),
            Err(_) => Err(())
        }
    }
}

// Builder for `age`.
impl<T1, T3> PersonBuilder<T1, (), T3> {
    fn age(self, value: i32) -> PersonBuilder<T1, i32, T3> {
        PersonBuilder {
            name: self.name,
            age: Some(value),
            gender: self.gender,
            _phantom: PhantomData,
        }
    }
}

// Builder for `gender`.
impl<T1, T2> PersonBuilder<T1, T2, ()> {
    fn gender(self, value: Gender) -> PersonBuilder<T1, T2, Gender> {
        PersonBuilder {
            name: self.name,
            age: self.age,
            gender: Some(value),
            _phantom: PhantomData,
        }
    }
}

// `build` function
// It can be called regardless of whether `T3` is `()` or `Gender`.
impl<T3> PersonBuilder<String, i32, T3> {
    fn build(self) -> Person {
        Person {
            name: self.name.unwrap(),
            age: self.age.unwrap(),
            gender: self.gender.unwrap(),
        }
    }
}
```
