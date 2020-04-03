#[macro_use]
extern crate qt5qml;
use qt5qml::core::{QApplicationFactory, QHashIntQByteArray, QModelIndex, QVariant, QT_USER_ROLE};
use qt5qml::gui::QGuiApplication;
use qt5qml::qml::QQmlApplicationEngine;
use std::convert::TryFrom;
use std::process::exit;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    qobject: *mut TestObject,
    items: Vec<(String, String)>,
}

const NAME_ROLE: i32 = QT_USER_ROLE;
const DESCRIPTION_ROLE: i32 = NAME_ROLE + 1;

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self {
            qobject,
            items: vec![
                ("Item 1".into(), "a description".into()),
                ("Item 2".into(), "a second description".into()),
            ],
        }
    }

    pub fn row_count(&self, parent: &QModelIndex) -> i32 {
        if parent.is_valid() {
            0
        } else {
            self.items.len() as i32
        }
    }

    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        if !index.is_valid() {
            return QVariant::new();
        }

        if let Some(item) = self.items.get(index.row() as usize) {
            match role {
                NAME_ROLE => (&item.0 as &str).into(),
                DESCRIPTION_ROLE => (&item.1 as &str).into(),
                _ => QVariant::new(),
            }
        } else {
            QVariant::new()
        }
    }

    pub fn role_names(&self) -> QHashIntQByteArray {
        let mut result = QHashIntQByteArray::new();
        result.insert(NAME_ROLE, "name".into());
        result.insert(DESCRIPTION_ROLE, "description".into());
        result
    }

    pub fn add_item(&mut self, name: &str, description: &str) {
        self.items.push((name.into(), description.into()))
    }
}

fn main() {
    let app = QGuiApplication::new_from_env_args();
    let mut engine = QQmlApplicationEngine::new(None);

    TestObject::register_type(cstr!("qobject_rs.test"), 1, 0, cstr!("TestObject"));

    engine.load("examples/listmodel/src/main.qml");
    if engine.root_objects().is_empty() {
        exit(-1);
    }

    exit(app.exec());
}
