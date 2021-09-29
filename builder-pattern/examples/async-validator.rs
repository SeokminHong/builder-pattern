use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[setter(async, value)]
    pub a: i32,
    #[default(5)]
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

#[tokio::main]
async fn main() {
    let t1 = Test::new().a(3).c(1).unwrap().build();
    println!("{:?}", t1);
    assert_eq!(t1, Ok(Test { a: 3, b: 5, c: 1 }));

    let t2 = Test::new()
        .a_async(|| async { 4 })
        .c(1)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t2);
    assert_eq!(t2, Ok(Test { a: 4, b: 5, c: 1 }));

    let t3 = Test::new()
        .a_async(|| async { 4 })
        .b_async(value)
        .c(1)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t3);
    assert!(t3.is_err());

    // Validator should be applied for default value.
    let t4 = Test::new()
        .a_async(|| async { 4 })
        .b_async(|| async { 5 })
        .build()
        .await;
    println!("{:?}", t4);
    assert!(t4.is_err());
}
