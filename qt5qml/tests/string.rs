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
