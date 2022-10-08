#[test]
fn shadowing() {
    let x = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);

    let x = 32;
    println!("{}", x); //cargo test -- --nocapture
}

#[test]
fn destructuring() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}
