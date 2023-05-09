use builder_pattern::Builder;

#[derive(Builder)]
pub struct Whatever {
    #[default(|i| -i)]
    pub bar: fn(i8) -> i8,
}

#[test]
fn default_fn() {
    let a = Whatever::new().build();
    assert_eq!((a.bar)(-1), 1);
}
