use builder_pattern::Builder;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq)]
pub enum MyEnum {
    A,
    B(i32),
}

// Private structure
#[derive(Builder, Debug, PartialEq)]
struct Test1<'a, 'b: 'a, T: Sized, U>
where
    U: Clone,
{
    pub a: T,
    #[validator(b_validator)]
    pub b: Cow<'a, U>,
    c: &'b &'static i32,
}

fn b_validator<'a, T: Clone>(v: Cow<'a, T>) -> Result<Cow<'a, T>, &'static str> {
    Ok(v)
}

#[derive(Builder, Debug, PartialEq, Eq)]
pub struct Test2 {
    pub a: i32,
    #[default(None)]
    pub b: Option<i32>,
    #[default(MyEnum::B(3))]
    c: MyEnum,
}

#[test]
fn complex_types() {
    let a = Test1::<i32, String>::new()
        .a(5)
        .b(Cow::Owned(String::from("Hello")))
        .unwrap()
        .c(&&3)
        .build();
    let b = Test2::new().a(5).build();

    println!("{:?}", a);
    assert_eq!(
        a,
        Test1::<i32, String> {
            a: 5,
            b: Cow::Owned(String::from("Hello")),
            c: &&3,
        }
    );
    println!("{:?}", b);
    assert_eq!(
        b,
        Test2 {
            a: 5,
            b: None,
            c: MyEnum::B(3),
        }
    );
}
