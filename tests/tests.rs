// #[cfg(test)]
// mod tests;

#[test]
fn test_greeting() {
    let res = pince::make_greeting("Tom");
    assert_eq!(res, "Hello, Tom!");
}

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

