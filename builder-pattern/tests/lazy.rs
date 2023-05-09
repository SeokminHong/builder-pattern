use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Person {
    #[setter(value, lazy, async)]
    name: String,
    age: u8,
    /// Default value is lazy evaluated.
    /// Only lazy setter is provided.
    #[default_lazy(|| "Seoul")]
    #[setter(lazy)]
    #[validator(is_not_empty)]
    address: &'static str,
}

fn is_not_empty(name: &'static str) -> Result<&'static str, &'static str> {
    if name.is_empty() {
        Err("Name cannot be empty.")
    } else {
        Ok(name)
    }
}

fn test_city() -> &'static str {
    "Tokyo"
}

#[test]
fn lazy() {
    tokio_test::block_on(lazy_impl());
}

async fn lazy_impl() {
    // `name` is evaluated here
    let a_builder = Person::new().name(String::from("Jack")).age(30);
    let a = a_builder.build(); // `address` is evaluated here
    println!("{:?}", a);
    assert_eq!(
        a,
        Person {
            name: String::from("Jack"),
            age: 30,
            address: "Seoul"
        }
    );

    let b_surname = "Johanson";
    // Lazy builder
    let b_builder = Person::new()
        .name_lazy(move || format!("Jane {}", b_surname))
        .age(50)
        .address_lazy(|| "New York");
    let b = b_builder.build(); // `name` and `address` is evaluated here
    println!("{:?}", b);
    assert_eq!(
        b,
        Ok(Person {
            name: String::from("Jane Johanson"),
            age: 50,
            address: "New York"
        })
    );

    // Asynchronous builder
    let c_builder = Person::new()
        .name_async(|| async { String::from("Joe") })
        .age(17)
        .address_lazy(test_city);
    let c = c_builder.build().await; // `name` and `address` is evaluated here
    println!("{:?}", c);
    assert_eq!(
        c,
        Ok(Person {
            name: String::from("Joe"),
            age: 17,
            address: "Tokyo"
        })
    );

    let d_builder = Person::new()
        .name_lazy(|| String::from("Jessica"))
        .age(50)
        .address_lazy(|| "");
    let d = d_builder.build(); // `name` and `address` is evaluated here
    println!("{:?}", d);
    assert!(d.is_err());
}
