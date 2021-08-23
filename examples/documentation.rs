use builder_pattern::Builder;

/// A structure describing a person.
/// ```
/// # use builder_pattern::Builder;
/// # #[derive(Builder, Debug)]
/// # struct Person {
/// #     pub name: String,
/// #     pub age: u8,
/// # }
/// let person  = Person::new()
///     .name("John")
///     .age(42)
///     .build();
///
/// println!("{:?}", person);
/// ```
#[derive(Builder, Debug)]
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

fn main() {
    let person = Person::new()
        .name(String::from("John"))
        .age(21)
        .city("Seoul")
        .build();
    println!("{:?}", person);
}
