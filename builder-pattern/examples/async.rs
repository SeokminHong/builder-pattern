use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[setter(async, value)]
    pub a: i32,
    #[default(5)]
    #[setter(async)]
    pub b: i32,
}

#[tokio::main]
async fn main() {
    let t1 = Test::new().a(3).build();
    assert_eq!(t1, Test { a: 3, b: 5 });
}
