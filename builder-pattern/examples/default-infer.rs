use builder_pattern::Builder;
use std::marker::PhantomData;

#[allow(unused)]
#[derive(Builder)]
struct Inferred<A = f64, B = String, F: FnMut(B) -> B = fn(B) -> B> {
    #[infer(A)]
    a: A,
    #[infer(B)]
    b: B,
    #[late_bound_default]
    #[default(|x| x)]
    b_default: F,
}

fn main() {
    let i = Inferred::new().b(String::new()).a("5").build();
}
