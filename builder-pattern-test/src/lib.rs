use builder_pattern::*;

#[derive(Builder)]
pub struct Test {
    pub a: i32,
    #[default(String::from(""))]
    pub b: String,
    #[default(None)]
    pub c: Option<i32>,
    pub d: Option<String>,
}

pub fn test() {
    let _t1 = Test::new().a(3).d(Some("foo".to_string())).build();
    let _t2 = Test::new()
        .a(3)
        .d(Some("foo".to_string()))
        .c(Some(3))
        .build();
}
