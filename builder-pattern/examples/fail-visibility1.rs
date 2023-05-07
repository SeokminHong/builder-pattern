mod test {
    use builder_pattern::Builder;

    // Private structure
    #[derive(Builder, Debug)]
    pub struct PrivateTest {
        pub a: i32,
        pub b: &'static str,
        c: i32,
    }
}

use test::*;

pub fn main() {
    let t1 = PrivateTest::new().a(333).c(1.234).b("hello").build();
}
