use std::any::TypeId;

use builder_pattern::Builder;

#[allow(unused)]
#[derive(Builder)]
struct Test<T = f64> {
    // Note that without the #[infer(T)], we would still have T = f64 from the
    // type param default.
    // The setter method will have a `T_` parameter, and return a TestBuilder<T_, ...>.
    #[infer(T)]
    #[into]
    vector: Vec<T>,
}

fn main() {
    let _ = Test::new().vector(&b"byte slice"[..]).build();

    // in more detail:
    // we can't use a mutable builder and re-assign it, because they are different types.
    let builder = Test::new();
    let builder = builder.vector::<u8, _>(&b"hello"[..]);
    let t = builder.build();
    assert_eq!(std::any::Any::type_id(&t.vector), TypeId::of::<Vec<u8>>());
}
