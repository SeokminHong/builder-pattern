mod test;

use test::*;

pub fn test() {
    let _t1 = Test::new().a(3).d(Some("foo".to_string())).build();
    let _t2 = Test::new()
        .a(3)
        .d(Some("foo".to_string()))
        .c(Some(3))
        .build();
}
