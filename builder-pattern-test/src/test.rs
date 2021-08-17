use builder_pattern::*;

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
}

#[allow(dead_code)]
#[derive(Builder, Debug)]
struct PrivateTest<'a, T: Sized, U>
where
    U: Clone,
{
    pub a: T,
    pub b: std::borrow::Cow<'a, U>,
    #[default(String::from(""))]
    pub c: String,
}

#[derive(Builder, Debug)]
pub struct PublicTest {
    pub a: i32,
    #[default(String::from(""))]
    pub b: String,
    #[default(None)]
    pub c: Option<i32>,
    #[allow(dead_code)]
    d: MyEnum,
}
