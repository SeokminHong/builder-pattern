use futures::{future::LocalBoxFuture, Future};
use std::{marker::PhantomData, panic};

enum Setter<'a, T> {
    Value(T),
    Lazy(Box<dyn 'a + Fn() -> T>),
    Async(Box<dyn 'a + Fn() -> LocalBoxFuture<'a, T>>),
}

struct AsyncBuilder {}

#[derive(Debug)]
struct Person {
    // #[setter(value, lazy, async)]
    name: String,
    age: u8,
    // Default value is lazy evaluated.
    // Only lazy setter is provided.
    // #[default_lazy(|| "Hello")]
    // #[setter(lazy)]
    address: &'static str,
}

struct PersonBuilder<'a, AsyncField, T1, T2, T3> {
    name: Option<Setter<'a, String>>,
    age: Option<Setter<'a, u8>>,
    address: Option<Setter<'a, &'static str>>,
    _phantom: PhantomData<(AsyncField, T1, T2, T3)>,
}

impl Person {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a>() -> PersonBuilder<'a, (), (), (), ()> {
        PersonBuilder {
            name: None,
            age: None,
            address: Some(Setter::Lazy(Box::new(|| "Seoul"))),
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
    ) -> PersonBuilder<'a, AsyncBuilder, String, T2, T3> {
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
            address: Some(Setter::Lazy(Box::new(value))),
            _phantom: PhantomData,
        }
    }
}

impl<'a, T3> PersonBuilder<'a, (), String, u8, T3> {
    pub fn build(self) -> Person {
        Person {
            name: match self.name.unwrap() {
                Setter::Value(v) => v,
                Setter::Lazy(f) => f(),
                _ => panic!(""),
            },
            age: match self.age.unwrap() {
                Setter::Value(v) => v,
                _ => panic!(""),
            },
            address: match self.address.unwrap() {
                Setter::Lazy(f) => f(),
                _ => panic!(""),
            },
        }
    }
}

impl<'a, T3> PersonBuilder<'a, AsyncBuilder, String, u8, T3> {
    pub async fn build(self) -> Person {
        Person {
            name: match self.name.unwrap() {
                Setter::Async(f) => f().await,
                Setter::Lazy(f) => f(),
                Setter::Value(v) => v,
            },
            age: match self.age.unwrap() {
                Setter::Value(v) => v,
                _ => panic!(""),
            },
            address: match self.address.unwrap() {
                Setter::Lazy(f) => f(),
                _ => panic!(""),
            },
        }
    }
}

#[tokio::main]
async fn main() {
    // `name` is evaluated here
    let a_builder = Person::new().name(String::from("Jack")).age(30);
    let a = a_builder.build(); // `address` is evaluated here
    println!("{:?}", a);

    let b_surname = "Johanson";
    // Lazy builder
    let b_builder = Person::new()
        .name_lazy(move || format!("Jane {}", b_surname))
        .age(50)
        .address_lazy(|| "New York");
    let b = b_builder.build(); // `name` and `address` is evaluated here
    println!("{:?}", b);

    // Asynchronous builder
    let c_builder = Person::new()
        .name_async(|| async { String::from("Joe") })
        .age(17)
        .address_lazy(|| "Tokyo");
    let c = c_builder.build().await; // `name` and `address` is evaluated here
    println!("{:?}", c);
}
