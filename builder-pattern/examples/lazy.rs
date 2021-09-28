//use builder_pattern::setter::*;
use builder_pattern::Builder;
//use std::future::Future;
//use std::marker::PhantomData;

#[derive(Builder, Debug)]
struct Person {
    #[setter(value, lazy, async)]
    name: String,
    age: u8,
    // Default value is lazy evaluated.
    // Only lazy setter is provided.
    // #[default_lazy(|| "Seoul")]
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

/*
struct PersonBuilder<'a, AsyncField, T1, T2, T3> {
    name: Option<Setter<'a, String>>,
    age: Option<Setter<'a, u8>>,
    address: Option<ValidatedSetter<'a, &'static str>>,
    _phantom: PhantomData<(AsyncField, T1, T2, T3)>,
}

impl Person {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a>() -> PersonBuilder<'a, (), (), (), ()> {
        #[allow(clippy::redundant_closure_call)]
        PersonBuilder {
            name: None,
            age: None,
            address: Some(ValidatedSetter::Lazy(Box::new(|| {
                is_not_empty((|| "Seoul")())
            }))),
            _phantom: PhantomData,
        }
    }
}

impl<'a, AsyncField, T2, T3> PersonBuilder<'a, AsyncField, (), T2, T3> {
    pub fn name(self, value: String) -> PersonBuilder<'a, AsyncField, String, T2, T3> {
        PersonBuilder {
            name: Some(Setter::Value(value)),
            age: self.age,
            address: self.address,
            _phantom: PhantomData,
        }
    }
    pub fn name_lazy<ValType: 'a + Fn() -> String>(
        self,
        value: ValType,
    ) -> PersonBuilder<'a, AsyncField, String, T2, T3> {
        PersonBuilder {
            name: Some(Setter::Lazy(Box::new(value))),
            age: self.age,
            address: self.address,
            _phantom: PhantomData,
        }
    }
}

impl<'a, AsyncField, T2, T3> PersonBuilder<'a, AsyncField, (), T2, T3> {
    pub fn name_async<
        ReturnType: 'a + Future<Output = String>,
        ValType: 'a + Fn() -> ReturnType,
    >(
        self,
        value: ValType,
    ) -> PersonBuilder<'a, AsyncBuilderMarker, String, T2, T3> {
        PersonBuilder {
            name: Some(Setter::Async(Box::new(move || Box::pin(value())))),
            age: self.age,
            address: self.address,
            _phantom: PhantomData,
        }
    }
}

impl<'a, AsyncField, T1, T3> PersonBuilder<'a, AsyncField, T1, (), T3> {
    pub fn age(self, value: u8) -> PersonBuilder<'a, AsyncField, T1, u8, T3> {
        PersonBuilder {
            name: self.name,
            age: Some(Setter::Value(value)),
            address: self.address,
            _phantom: PhantomData,
        }
    }
}

impl<'a, AsyncField, T1, T2> PersonBuilder<'a, AsyncField, T1, T2, ()> {
    pub fn address_lazy<ValType: 'a + Fn() -> &'static str>(
        self,
        value: ValType,
    ) -> PersonBuilder<'a, AsyncField, T1, T2, &'static str> {
        PersonBuilder {
            name: self.name,
            age: self.age,
            address: Some(ValidatedSetter::Lazy(Box::new(move || {
                is_not_empty(value())
            }))),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T3> PersonBuilder<'a, (), String, u8, T3> {
    // If the struct needs lazy validation, it should return `Result`.
    pub fn build(self) -> Result<Person, &'static str> {
        let address = match match self.address.unwrap() {
            ValidatedSetter::Lazy(f) => f(),
            ValidatedSetter::Value(v) => Ok(v),
            _ => unreachable!(),
        } {
            Ok(address) => address,
            Err(e) => return Err(e),
        };
        Ok(Person {
            name: match self.name.unwrap() {
                Setter::Value(v) => v,
                Setter::Lazy(f) => f(),
                _ => unreachable!(),
            },
            age: match self.age.unwrap() {
                Setter::Value(v) => v,
                _ => unreachable!(),
            },
            address,
        })
    }
}

impl<'a, T3> PersonBuilder<'a, AsyncBuilderMarker, String, u8, T3> {
    pub async fn build(self) -> Result<Person, &'static str> {
        let address = match match self.address.unwrap() {
            ValidatedSetter::Async(f) => f().await,
            ValidatedSetter::Lazy(f) => f(),
            ValidatedSetter::Value(v) => Ok(v),
        } {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(Person {
            name: match self.name.unwrap() {
                Setter::Async(f) => f().await,
                Setter::Lazy(f) => f(),
                Setter::Value(v) => v,
            },
            age: match self.age.unwrap() {
                Setter::Value(v) => v,
                _ => unreachable!(),
            },
            address,
        })
    }
}
*/

#[allow(dead_code)]
fn test_city() -> &'static str {
    "Tokyo"
}

#[tokio::main]
async fn main() {
    // `name` is evaluated here
    //let a_builder = Person::new().name(String::from("Jack")).age(30);
    //let a = a_builder.build(); // `address` is evaluated here
    //println!("{:?}", a);

    let b_surname = "Johanson";
    // Lazy builder
    let b_builder = Person::new()
        .name_lazy(move || format!("Jane {}", b_surname))
        .age(50)
        .address_lazy(|| "New York");
    let b = b_builder.build(); // `name` and `address` is evaluated here
    println!("{:?}", b);

    // Asynchronous builder
    /*let c_builder = Person::new()
        .name_async(|| async { String::from("Joe") })
        .age(17)
        .address_lazy(test_city);
    let c = c_builder.build().await; // `name` and `address` is evaluated here
    println!("{:?}", c);*/

    let d_builder = Person::new()
        .name_lazy(move || format!("Jessica {}", b_surname))
        .age(50)
        .address_lazy(|| "");
    let d = d_builder.build(); // `name` and `address` is evaluated here
    println!("{:?}", d);
}
