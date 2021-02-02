use qt5qml::core::{QString, QStringList, ToQString};

#[test]
fn check_push() {
    let mut list = QStringList::new();

    list.push("test".to_qstring());

    assert_eq!(1, list.len());
    assert_eq!("test".to_qstring(), list.as_slice()[0]);
}

#[test]
fn check_extend_from_slice() {
    let mut list = QStringList::new();

    list.extend_from_slice(&["1".to_qstring(), "2".to_qstring()]);
    list.extend_from_slice(&["3".to_qstring(), "4".to_qstring()]);

    assert_eq!(4, list.len());
    let slice = list.as_slice();
    assert_eq!("1".to_qstring(), slice[0]);
    assert_eq!("2".to_qstring(), slice[1]);
    assert_eq!("3".to_qstring(), slice[2]);
    assert_eq!("4".to_qstring(), slice[3]);
}

#[test]
fn check_collect() {
    let list: QStringList = vec!["1".to_qstring(), "2".to_qstring()]
        .into_iter()
        .collect();
    assert_eq!(2, list.len());
}

#[test]
fn check_collect_to_vec() {
    let mut list = QStringList::new();
    list.extend_from_slice(&["1".to_qstring(), "2".to_qstring()]);

    let vec: Vec<String> = list.iter().map(|s| s.to_string()).collect();
    assert_eq!(2, vec.len());
}
