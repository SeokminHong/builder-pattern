use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[default(-1)]
    #[validator(is_positive)]
    pub a: i32,
    pub b: i32,
}

fn is_positive(v: i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(v)
    } else {
        Err("Value is negative or zero.")
    }
}

fn main() {
    let t1 = Test::new().a(4).unwrap().b(5).build();
    println!("{:?}", t1);
    assert_eq!(t1, Test { a: 4, b: 5 });

    let t2 = Test::new().b(5).build();
    println!("{:?}", t2);
    assert!(t2.is_err());
}
