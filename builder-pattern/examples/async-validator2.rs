use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[validator(is_positive)]
    #[setter(async, value)]
    pub a: i32,
    #[validator(is_positive)]
    #[setter(async, value)]
    pub b: i32,
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
    // If only value setters are used for validating fields, results should not be `Result`.
    let t1 = Test::new().a(3).unwrap().b(3).unwrap().build();
    println!("{:?}", t1);
    assert_eq!(t1, Test { a: 3, b: 3 });

    let t2 = Test::new()
        .a_async(|| async { 3 })
        .b_async(|| async { 3 })
        .build()
        .await;
    println!("{:?}", t2);
    assert_eq!(t2, Ok(Test { a: 3, b: 3 }));

    let t3 = Test::new()
        .a_async(|| async { 3 })
        .b(3)
        .unwrap()
        .build()
        .await;
    println!("{:?}", t3);
    assert_eq!(t3, Ok(Test { a: 3, b: 3 }));
}
