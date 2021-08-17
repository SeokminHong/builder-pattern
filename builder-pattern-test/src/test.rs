use builder_pattern::*;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MyEnum {
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
    #[default(Some(3))]
    pub c: Option<i32>,
    #[allow(dead_code)]
    d: MyEnum,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_test() {
        let a = PrivateTest::<i32, String>::new()
            .a(5)
            .b(std::borrow::Cow::Owned(String::from("Hello")))
            .build();

        let b = PrivateTest::<i32, String>::new()
            .b(std::borrow::Cow::Owned(String::from("foo")))
            .a(3)
            .c(String::from("hi"))
            .build();

        print!("{:?} {:?}", a, b);
    }
}
