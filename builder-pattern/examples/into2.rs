use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[into]
    #[setter(value, lazy, async)]
    a: String,
}

#[tokio::main]
async fn main() {
    let a = Test::new().a("foo").build();
    println!("{:?}", a);
    assert_eq!(
        a,
        Test {
            a: String::from("foo"),
        }
    );

    let b = Test::new().a_lazy(|| "foo").build();
    println!("{:?}", b);
    assert_eq!(
        b,
        Test {
            a: String::from("foo"),
        }
    );

    let c = Test::new().a_async(|| async { "foo" }).build().await;
    println!("{:?}", c);
    assert_eq!(
        c,
        Test {
            a: String::from("foo"),
        }
    );
}
