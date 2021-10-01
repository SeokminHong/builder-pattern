use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[setter(lazy, value)]
    pub a: i32,
    #[default_lazy(|| -5)]
    #[validator(is_positive)]
    #[setter(lazy)]
    pub b: i32,
    #[default(-1)]
    #[validator(is_positive)]
    #[setter(value, lazy)]
    pub c: i32,
}

fn value() -> i32 {
    -10
}

fn is_positive(v: i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(v)
    } else {
        Err("Value is negative or zero.")
    }
}

fn main() {
    let t1 = Test::new()
        .a_lazy(|| 4)
        // `b` is an lazy setter with validator.
        // The result of the build should return `Result`.
        .b_lazy(|| 5)
        .c(1)
        .unwrap()
        .build();
    println!("{:?}", t1);
    assert_eq!(t1, Ok(Test { a: 4, b: 5, c: 1 }));

    let t2 = Test::new()
        .a_lazy(|| 4)
        .b_lazy(value) // Returns 10 asynchronously.
        .c(1)
        .unwrap()
        .build();
    println!("{:?}", t2);
    assert!(t2.is_err());

    // Validator doesn't work with default values.
    // So, b will be -5 instead of error.
    let t3 = Test::new().a(3).c(1).unwrap().build();
    println!("{:?}", t3);
    assert_eq!(t3, Test { a: 3, b: -5, c: 1 });

    let t4 = Test::new().a_lazy(|| 4).b_lazy(|| 5).build();
    println!("{:?}", t4);
    assert_eq!(t4, Ok(Test { a: 4, b: 5, c: -1 }));
}
