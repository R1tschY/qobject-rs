use qt5qml::core::QVariant;
use std::convert::TryFrom;

#[test]
fn variant_debug() {
    assert_eq!(
        format!("{:?}", QVariant::from("ABC")),
        "QVariant(QString, \"ABC\")"
    );
    assert_eq!(format!("{:?}", QVariant::new()), "QVariant(Invalid)");
}

#[test]
fn variant_valid() {
    assert!(!QVariant::new().is_valid());
    assert!(QVariant::from(123u8).is_valid());
}

#[test]
fn variant_from() {
    // string
    assert_eq!(String::from(&QVariant::from("test")), "test");
    assert_eq!(String::from(QVariant::from("test")), "test");

    // optional string
    let none_string: Option<String> = None;
    assert_eq!(String::from(QVariant::from(Some("test"))), "test");
    assert_eq!(String::from(QVariant::from(none_string.clone())), "");
    assert!(QVariant::from(none_string).is_null());

    // numbers
    assert_eq!(u8::try_from(QVariant::from(123u8)).unwrap(), 123u8);
    assert_eq!(i8::try_from(QVariant::from(-123i8)).unwrap(), -123i8);
    assert_eq!(u16::try_from(QVariant::from(123u16)).unwrap(), 123u16);
    assert_eq!(i16::try_from(QVariant::from(-123i16)).unwrap(), -123i16);
    assert_eq!(
        i32::try_from(QVariant::from(-123456i32)).unwrap(),
        -123456i32
    );
    assert_eq!(u32::try_from(QVariant::from(123456u32)).unwrap(), 123456u32);
    assert_eq!(
        i64::try_from(QVariant::from(-123456i32)).unwrap(),
        -123456i64
    );
    assert_eq!(u64::try_from(QVariant::from(123456u32)).unwrap(), 123456u64);
    assert_eq!(
        f32::try_from(QVariant::from(12345.6f32)).unwrap(),
        12345.6f32
    );
    assert_eq!(
        f64::try_from(QVariant::from(12345.6f64)).unwrap(),
        12345.6f64
    );
    assert_eq!(u8::try_from(QVariant::new()), Err(()));
    assert_eq!(u8::try_from(QVariant::from("test")), Err(()));
}
