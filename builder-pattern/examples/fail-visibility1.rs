mod test {
    use builder_pattern::Builder;

    // Private structure
    #[derive(Builder, Debug)]
    struct PrivateTest {
        pub a: i32,
        pub b: &'static str,
        c: i32,
    }
}

pub fn main() {
    let t1 = PrivateTest::new().a(333).c(1.234).b("hello").build();
}
