#![allow(dead_code)]

mod test {
    use builder_pattern::Builder;

    // Public structure
    #[derive(Builder, Debug)]
    #[allow(unused)]
    pub struct PublicTest {
        pub a: i32,
        pub b: Option<i32>,
        #[default(1.234)]
        #[public]
        c: f64,
    }
}

use test::*;

#[test]
pub fn visibility() {
    let t1 = PublicTest::new().a(333).b(Some(123)).c(3.1).build();
    println!("{:?}", t1);
}
