# builder-pattern

A derivable macro for declaring a builder pattern.

## Examples

```rust
use builder_pattern::*;

// Derive `Builder` macro for a structure.
#[derive(Builder)]
pub struct Test {
    pub a: i32,
    #[default(String::from(""))]
    pub b: String,
    #[default(None)]
    pub c: Option<i32>,
    pub d: bool,
}

pub fn test() {
    let _t1 = Test::new().a(3).d(false).build();
    let _t2 = Test::new()
        .a(3)
        .d(true)
        .c(Some(3))
        .build();
    // Compile error
    let _t3 = Test::new().a(5).build();
}
```

It considers all fields without `default` attribute as required. If the attribute is provided, the expression in the parantheses is evaluated as a default value.

When the insuficient number of arguments is provided, the compilation will fail.
