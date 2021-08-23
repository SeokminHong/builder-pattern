use builder_pattern::Builder;

#[derive(Builder, Debug)]
struct Test {
    #[default(String::from("Jack"))]
    name: String,
    #[default(19)]
    age: u8,
}

fn main() {
    let t1 = Test::new().build();
    let t2 = Test::new().name(String::from("Jane")).build();
    let t3 = Test::new().age(31).name(String::from("Jane")).build();

    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
}
