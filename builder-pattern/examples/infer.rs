use builder_pattern::Builder;
use std::any::{Any, TypeId};

#[allow(unused)]
#[derive(Builder)]
struct Op<T = f64> {
    #[infer(T)]
    a: Option<T>,
    #[infer(T)]
    b: T,
}

fn main() {
    let a = Op::new().a(Some(5i32)).b(1).build();
    assert_eq!(a.type_id(), TypeId::of::<Op<i32>>());
}
