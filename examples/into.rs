use builder_pattern::Builder;

#[derive(Builder, Debug)]
struct Test {
    a: i32,
    #[setter(into)]
    b: String,
    c: String,
}

fn main() {
    let a = Test::new()
        .a(3)
        // Implicit conversion (&str -> String)
        .b("foo")
        // Implicit conversion is not allowed.
        // passing `"bar"` will fail.
        .c(String::from("bar"))
        .build();
    println!("{:?}", a);
}
