use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[setter(async, value)]
    pub a: i32,
    #[default(-5)]
    #[validator(is_positive)]
    #[setter(async)]
    pub b: i32,
    #[default(-1)]
    #[validator(is_positive)]
    #[setter(value, async)]
    pub c: i32,
}

async fn value() -> i32 {
    -10
}

fn is_positive(v: i32) -> Result<i32, &'static str> {
    if v > 0 {
        Ok(v)
    } else {
        Err("Value is negative or zero.")
    }
}

#[test]
fn async_validator() {
    tokio_test::block_on(async_validator_impl());
}

async fn async_validator_impl() {
    let t1 = Test::new()
        .a_async(|| async { 4 })
        // `b` is an async setter with validator.
        // The result of the build should return `Result`.
        .b_async(|| async { 5 })
        .c(1)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t1);
    assert_eq!(t1, Ok(Test { a: 4, b: 5, c: 1 }));

    let t2 = Test::new()
        .a_async(|| async { 4 })
        .b_async(value) // Returns 10 asynchronously.
        .c(1)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t2);
    assert!(t2.is_err());

    // Validator doesn't work with default values.
    // So, b will be -5 instead of error.
    let t3 = Test::new().a(3).c(1).unwrap().build();
    println!("{:?}", t3);
    assert_eq!(t3, Test { a: 3, b: -5, c: 1 });

    let t4 = Test::new()
        .a_async(|| async { 4 })
        .b_async(|| async { 5 })
        .build()
        .await;
    println!("{:?}", t4);
    assert_eq!(t4, Ok(Test { a: 4, b: 5, c: -1 }));
}
