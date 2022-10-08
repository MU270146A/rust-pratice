
#[test]
fn take_ownership() {
    let s1 = String::from("hello, world");
    let s2 = take(&s1);
    println!("{}", s2);
    println!("{}", s1);
}

fn take(s: &String) -> &String{
    println!("{}", s);
    s
}

#[test]
fn mutability() {
    let x = Box::new(5);
    let mut y = Box::new(4);
    *y = 4;
    assert_eq!(*x, 5);
}

#[test]
fn reference() {
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
}

#[test]
fn borrowing_value() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &r1;
    println!("{}, {}", r1, r2);
}
