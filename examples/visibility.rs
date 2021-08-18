mod vis_mod {
    use builder_pattern::Builder;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum MyEnum {
        A,
        B(i32),
    }

    #[allow(dead_code)]
    #[derive(Builder, Debug)]
    struct PrivateTest<'a, 'b: 'a, T: Sized, U>
    where
        U: Clone,
    {
        pub a: T,
        pub b: std::borrow::Cow<'a, U>,
        #[default(String::from(""))]
        pub c: String,
        pub d: &'b &'static i32,
    }

    #[derive(Builder, Debug)]
    pub struct PublicTest {
        pub a: i32,
        #[default(String::from(""))]
        #[setter(into)]
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
                .d(&&3)
                .build();

            let b = PrivateTest::<i32, String>::new()
                .d(&&4)
                .b(std::borrow::Cow::Owned(String::from("foo")))
                .a(3)
                .c(String::from("hi"))
                .build();

            print!("{:?} {:?}", a, b);
        }
    }
}

use vis_mod::*;

pub fn main() {
    let t1 = PublicTest::new()
        .a(333)
        .d(MyEnum::B(5123))
        .b("Test")
        .build();
    println!("{:?}", t1);
}
