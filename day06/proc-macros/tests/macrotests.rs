use proc_macros::*;

#[serialize(json, 2)]
struct Player {
    id: i32,
    name: String,
    level: u8,
}

#[test]
fn should_serialize_works_test() {
    let _marine = Player {
        id: 2,
        name: "Marine Corp".to_owned(),
        level: 10,
    };
}
