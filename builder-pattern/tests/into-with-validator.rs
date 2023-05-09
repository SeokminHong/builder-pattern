use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    #[into]
    #[validator(is_empty)]
    #[setter(value, lazy, async)]
    a: String,
}

fn is_empty(value: String) -> Result<String, &'static str> {
    if value.is_empty() {
        Err("Value is empty")
    } else {
        Ok(value)
    }
}

#[test]
fn into_with_validator() {
    tokio_test::block_on(into_with_validator_impl());
}

async fn into_with_validator_impl() {
    let a = Test::new().a("foo").unwrap().build();
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
        Ok(Test {
            a: String::from("foo"),
        })
    );

    let c = Test::new().a_async(|| async { "foo" }).build().await;
    println!("{:?}", c);
    assert_eq!(
        c,
        Ok(Test {
            a: String::from("foo"),
        })
    );
}
