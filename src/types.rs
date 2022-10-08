
#[test]
fn string() {
    let s = String::from("hi,中国");
    let h = &s[0..1];
    assert_eq!(h, "h");
}

#[test]
fn arr() {
    let names = [String::from("hello"), String::from("world")];
    let first = names.get(0).unwrap();
    let second = &names[0]; //not safe
}


#[test]
fn make_struct() {
    struct Person {
        name: String,
        age: u8,
        hobby: String
    }

    let age = 30;
    let mut p = Person {
        name: String::from("chirs"),
        age: age,
        hobby: "coding".to_string()
    };

    p.age = 10;
    p.name = String::from("Lebron");
}

#[test]
fn make_enum() {


}