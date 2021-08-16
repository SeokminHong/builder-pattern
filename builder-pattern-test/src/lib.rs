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
