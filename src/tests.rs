// Print integer values using the generic function
#[test]
fn test_print_integer_value() {
    let mut output = Vec::new();
    let expected = "Value is 42\n";

    std::io::set_print(Some(Box::new(&mut output)));
    print_value(42);
    std::io::set_print(None);

    let actual = String::from_utf8(output).unwrap();
    assert_eq!(actual, expected);
}

// Pass empty string as input
#[test]
fn test_print_empty_string() {
    let mut output = Vec::new();
    let expected = "Value is \n";

    std::io::set_print(Some(Box::new(&mut output)));
    print_value("");
    std::io::set_print(None);

    let actual = String::from_utf8(output).unwrap();
    assert_eq!(actual, expected);
}
