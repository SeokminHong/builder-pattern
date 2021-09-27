use futures::{future::LocalBoxFuture, Future};
use std::{marker::PhantomData, panic};

enum Setter<'a, T> {
    Value(T),
    Lazy(fn() -> T),
    Async(Box<dyn 'a + Fn() -> LocalBoxFuture<'a, T>>),
}

struct AsyncBuilder {}

#[derive(Debug)]
struct Person {
    // #[setter(value, lazy, async)]
    name: String,
    age: u8,
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
            address: Some(Setter::Lazy(|| "Seoul")),
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
    pub fn name_lazy(self, value: fn() -> String) -> PersonBuilder<'a, AsyncField, String, T2, T3> {
        PersonBuilder {
            name: Some(Setter::Lazy(value)),
            age: self.age,
            address: self.address,
            _phantom: PhantomData,
        }
    }
}

impl<'a, AsyncField, T2, T3> PersonBuilder<'a, AsyncField, (), T2, T3> {
    pub fn name_async<ReturnType>(
        self,
        value: fn() -> ReturnType,
    ) -> PersonBuilder<'a, AsyncBuilder, String, T2, T3>
    where
        ReturnType: Future<Output = String> + 'a,
    {
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
    pub fn address(
        self,
        value: fn() -> &'static str,
    ) -> PersonBuilder<'a, AsyncField, T1, T2, &'static str> {
        PersonBuilder {
            name: self.name,
            age: self.age,
            address: Some(Setter::Lazy(value)),
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
    // Synchronous builder
    let a_builder = Person::new()
        .name(String::from("Jack"))
        .age(30)
        .address(|| "New York");
    // Lazy builder
    let b_builder = Person::new().name_lazy(|| String::from("Jane")).age(50);
    // Asynchronous builder
    let c_builder = Person::new()
        .name_async(|| async { String::from("Joe") })
        .age(17)
        .address(|| "Tokyo");

    let a = a_builder.build();
    println!("{:?}", a);
    let b = b_builder.build();
    println!("{:?}", b);
    let c = c_builder.build().await;
    println!("{:?}", c);
}
