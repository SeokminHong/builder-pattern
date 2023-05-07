use builder_pattern::Builder;

#[derive(Builder)]
struct Thing {}

fn main() {
    let _: Thing = Thing::new().build();
}
