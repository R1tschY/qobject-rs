use qt5qml::core::QString;

#[test]
fn string_debug() {
    assert_eq!(format!("{:?}", QString::from("ABC")), "\"ABC\"");
    assert_eq!(format!("{:?}", QString::new()), "\"\"");
}

#[test]
fn string_from() {
    assert_eq!(QString::from("ABC").to_string(), String::from("ABC"));
    assert_eq!(
        QString::from(String::from("ABC")).to_string(),
        String::from("ABC")
    );
}

#[test]
fn string_from_empty() {
    assert!(QString::from("").is_empty());
    assert!(!QString::from("").is_null());
}

#[test]
fn string_init() {
    assert!(QString::new().is_empty());
    assert!(QString::new().is_null());
}
