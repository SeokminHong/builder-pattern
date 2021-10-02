use builder_pattern::Builder;

#[derive(Builder, Debug, PartialEq)]
struct Test {
    a: i32,
    #[into]
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
    assert_eq!(
        a,
        Test {
            a: 3,
            b: "foo".to_string(),
            c: "bar".to_string()
        }
    );
}
