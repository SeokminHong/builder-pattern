use builder_pattern::Builder;

#[allow(unused)]
#[derive(Builder)]
struct Op<T = &'static str> {
    #[replace_generics(T)]
    #[default(None)]
    optional_field: Option<T>,
}

fn main() {
    use std::any::{Any, TypeId};
    // Should be inferred as Op<&'static str>, i.e. the macro should notice the defaulted type param.
    let a = Op::new().build();
    assert_eq!(Any::type_id(&a), TypeId::of::<Op<&'static str>>());

    // Should be inferred as Op<i32>
    let a = Op::new().optional_field(Some(5i32)).build();
    assert_eq!(Any::type_id(&a), TypeId::of::<Op<i32>>());
}
