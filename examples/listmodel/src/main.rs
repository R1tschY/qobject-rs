use qt5qml::core::{QCoreApplicationFactory, QModelIndex, QVariant};
use qt5qml::gui::QGuiApplication;
use qt5qml::qml::QQmlApplicationEngine;
use std::process::exit;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    qobject: *mut TestObject,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self { qobject }
    }

    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        0
    }

    pub fn data(&self, parent: &QModelIndex, role: i32) -> QVariant {
        QVariant::new()
    }
}

fn main() {
    let app = QGuiApplication::new_from_env_args();
    let mut engine = QQmlApplicationEngine::new(None);
    engine.load("examples/listmodel/src/main.qml");
    if engine.root_objects().is_empty() {
        exit(-1);
    }

    exit(app.exec());
}
