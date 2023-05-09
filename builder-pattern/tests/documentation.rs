#![allow(dead_code)]
use builder_pattern::Builder;

/// A structure describing a person.
/// ```
/// # use builder_pattern::Builder;
/// # #[derive(Builder, Debug)]
/// # struct Person {
/// #     pub name: String,
/// #     pub age: u8,
/// #     pub city: String,
/// # }
/// let person  = Person::new()
///     .name("John")
///     .age(42)
///     .city("New York")
///     .build();
///
/// println!("{:?}", person);
/// ```
#[derive(Builder, Debug, PartialEq)]
struct Person {
    /**
     * Name of the person.
     * Required field.
     */
    pub name: String,
    /// Age of the person.
    #[default(19)]
    pub age: u8,
    #[doc = r" Multi-line comments...
    May span many lines"]
    #[into]
    pub city: String,
}

#[test]
fn main() {
    let person = Person::new()
        .name(String::from("John"))
        .age(21)
        .city("Seoul")
        .build();

    assert_eq!(
        person,
        Person {
            name: String::from("John"),
            age: 21,
            city: String::from("Seoul"),
        }
    )
}
