mod test;

use test::*;

pub fn main() {
    let t1 = PublicTest::new()
        .a(3)
        .d(MyEnum::B(5))
        .b(String::from("Test"))
        .build();
    println!("{:?}", t1);
}
