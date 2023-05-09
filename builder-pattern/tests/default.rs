#![allow(dead_code)]

use builder_pattern::Builder;
use uuid::Uuid;

#[allow(unused)]
#[derive(Builder, Debug)]
struct Test {
    #[default(String::from("Jack"))]
    name: String,
    #[default(19)]
    age: u8,
    #[default(Uuid::new_v4())]
    #[hidden]
    id: Uuid,
}

impl PartialEq for Test {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

#[test]
fn default() {
    let t1 = Test::new().build();
    let t2 = Test::new().name(String::from("Jane")).build();
    let t3 = Test::new().age(31).name(String::from("Jane")).build();

    println!("{:?}", t1);
    assert_eq!(
        t1,
        Test {
            name: String::from("Jack"),
            age: 19,
            id: Uuid::new_v4()
        }
    );

    println!("{:?}", t2);
    assert_eq!(
        t2,
        Test {
            name: String::from("Jane"),
            age: 19,
            id: Uuid::new_v4()
        }
    );

    println!("{:?}", t3);
    assert_eq!(
        t3,
        Test {
            name: String::from("Jane"),
            age: 31,
            id: Uuid::new_v4()
        }
    );
}
