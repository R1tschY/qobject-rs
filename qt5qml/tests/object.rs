use qt5qml::core::{ConnectionType, QObject, QObjectRef, QTimer};
use std::ptr;
use std::time::Duration;

#[test]
fn object_connect() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());

    assert!(!timer.is_active());
    drop(object);
    assert!(timer.is_active());
}

#[test]
fn object_connect_signal_signal() {
    let object1 = QObject::new(None);
    let object2 = QObject::new(None);

    let connection = QObject::connect(
        &object1,
        QObject::destroyed_signal(),
        &object2,
        QObject::destroyed_signal(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());
}

#[test]
fn object_disconnect_connection() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());
    assert!(connection.disconnect());
}

#[test]
fn object_disconnect_all() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());
    assert!(object.disconnect_from(timer.as_qobject()));
}

#[test]
fn object_disconnect_method() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());
    assert!(object.disconnect_from_method(timer.as_qobject(), QTimer::start_slot()));
}

#[test]
fn object_disconnect() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());

    let success = QObject::disconnect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
    );
    assert!(success);
}

#[test]
fn object_disconnect_nothing() {
    let object = QObject::new(None);
    let mut timer = QTimer::new(None);
    timer.set_interval(Duration::from_secs(10));

    let connection = QObject::connect(
        &object,
        QObject::object_name_changed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
        ConnectionType::default(),
    );
    assert!(connection.is_valid());

    let success = QObject::disconnect(
        &object,
        QObject::destroyed_signal(),
        timer.as_qobject(),
        QTimer::start_slot(),
    );
    assert!(!success);
}
