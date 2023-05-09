use builder_pattern::Builder;

#[allow(unused)]
#[derive(Builder)]
struct LateBound<A, B, F: FnMut(B) -> B = fn(B) -> B> {
    field_a: A,
    field_b: B,
    #[late_bound_default]
    #[default(|x| x)]
    transform_b: F,
}

impl<A, B> LateBound<A, B>
where
    B: Clone,
{
    fn get_b(&self) -> B {
        (self.transform_b)(self.field_b.clone())
    }
}

fn with() {
    let l = LateBound::new()
        .field_a(String::new())
        .field_b(5)
        .transform_b(|x| x + 10)
        .build();
    assert_eq!(l.get_b(), 15);
}
fn without() {
    let l = LateBound::new().field_a(String::new()).field_b(200).build();
    assert_eq!(l.get_b(), 200);
}

fn main() {
    with();
    without();
}
