use builder_pattern::Builder;
use std::cmp::Ordering;

#[derive(Builder, Debug)]
struct Test {
    #[validator(to_positive)]
    positive: i32,
}

fn to_positive(val: i32) -> Result<i32, &'static str> {
    match val.cmp(&0) {
        Ordering::Greater => Ok(val),
        Ordering::Less => Ok(-val),
        Ordering::Equal => Err("The input value is zero."),
    }
}

fn main() {
    let a = Test::new().positive(5).unwrap().build();
    let b = Test::new().positive(-5).unwrap().build();
    println!("a: {:?}, b: {:?}", a, b);

    if let Err(err) = Test::new().positive(0) {
        println!("{}", err);
    }
}
