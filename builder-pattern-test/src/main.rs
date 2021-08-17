mod test;

use test::*;

pub fn main() {
    let t1 = PublicTest::new()
        .a(333)
        .d(MyEnum::B(5123))
        .b("Test")
        .build();
    println!("{:?}", t1);
}
