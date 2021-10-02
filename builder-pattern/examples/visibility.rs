mod test {
    use builder_pattern::Builder;

    // Private structure
    #[derive(Builder, Debug)]
    struct PrivateTest {
        pub a: i32,
        pub b: &'static str,
        c: i32,
    }

    // Public structure
    #[derive(Builder, Debug)]
    pub struct PublicTest {
        pub a: i32,
        pub b: Option<i32>,
        c: f64,
    }

    pub fn private_test() {
        let a = PrivateTest::new().a(5).b("Hello").c(3).build();

        let b = PrivateTest::new().c(4).b("foo").a(3).build();

        println!("{:?}", a);
        println!("{:?}", b);
    }
}

use test::*;

pub fn main() {
    let t1 = PublicTest::new().a(333).c(1.234).b(Some(123)).build();
    println!("{:?}", t1);

    test::private_test();
}
