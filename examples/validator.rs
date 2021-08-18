use builder_pattern::*;
use std::cmp::Ordering;

#[derive(Builder, Debug)]
struct Test {
    #[validator(to_absolute)]
    positive: i32,
}

fn to_absolute(val: i32) -> Result<i32, ()> {
    match val.cmp(&0) {
        Ordering::Greater => Ok(val),
        Ordering::Less => Ok(-val),
        Ordering::Equal => Err(()),
    }
}

fn main() {
    let a = Test::new().positive(5).unwrap().build();
    let b = Test::new().positive(-5).unwrap().build();
    println!("a: {:?}, b: {:?}", a, b);

    if Test::new().positive(0).is_err() {
        println!("Invalid zero detected");
    }
}
