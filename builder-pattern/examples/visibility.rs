mod test {
    use builder_pattern::Builder;

    // Public structure
    #[derive(Builder, Debug)]
    pub struct PublicTest {
        pub a: i32,
        pub b: Option<i32>,
        #[default(1.234)]
        c: f64,
    }
}

use test::*;

pub fn main() {
    let t1 = PublicTest::new().a(333).b(Some(123)).build();
    println!("{:?}", t1);
}
