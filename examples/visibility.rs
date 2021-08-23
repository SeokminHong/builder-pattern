mod vis_mod {
    use builder_pattern::Builder;
    use std::borrow::Cow;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum MyEnum {
        A,
        B(i32),
    }

    // Private structure
    #[allow(dead_code)]
    #[derive(Builder, Debug)]
    struct PrivateTest<'a, 'b: 'a, T: Sized, U>
    where
        U: Clone,
    {
        pub a: T,
        pub b: Cow<'a, U>,
        c: &'b &'static i32,
    }

    // Public structure
    #[derive(Builder, Debug)]
    pub struct PublicTest {
        pub a: i32,
        pub b: Option<i32>,
        c: MyEnum,
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::borrow::Cow;

        #[test]
        fn private_test() {
            let a = PrivateTest::<i32, String>::new()
                .a(5)
                .b(Cow::Owned(String::from("Hello")))
                .c(&&3)
                .build();

            let b = PrivateTest::<i32, String>::new()
                .c(&&4)
                .b(Cow::Owned(String::from("foo")))
                .a(3)
                .build();

            println!("{:?}", a);
            println!("{:?}", b);
        }
    }
}

use vis_mod::*;

pub fn main() {
    let t1 = PublicTest::new()
        .a(333)
        .c(MyEnum::B(5123))
        .b(Some(123))
        .build();
    println!("{:?}", t1);
}
