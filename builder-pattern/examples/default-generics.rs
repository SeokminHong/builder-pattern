use builder_pattern::Builder;

#[allow(unused)]
#[derive(Builder)]
struct Op<T = &'static str> {
    #[replace_generics(T)]
    #[default(None)]
    optional_field: Option<T>,
}

fn main() {
    // Should be inferred as Op<&'static str>, i.e. the macro should notice the defaulted type param.
    let _ = Op::new().build();
    // Should be inferred as Op<i32>
    let _ = Op::new().optional_field(Some(5i32)).build();
}
