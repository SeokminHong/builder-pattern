use builder_pattern::Builder;
use std::marker::PhantomData;

#[allow(unused)]
#[derive(Builder)]
struct Inferred<A = f64, B = String> {
    #[infer(A)]
    a: A,
    #[infer(B)]
    b: B,
    // #[hidden]
    #[syntactic_default]
    #[default(PhantomData)]
    b_default: PhantomData<B>,
}

fn main() {
    let i = Inferred::new().a(5i32).b(String::new()).build();
}
