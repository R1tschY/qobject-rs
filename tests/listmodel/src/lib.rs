#![allow(unused)]

use qt5qml::core::{QModelIndex, QVariant};
include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    _qobject: *mut TestObject,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self { _qobject: qobject }
    }

    pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
        0
    }

    pub fn data(&self, _parent: &QModelIndex, _role: i32) -> QVariant {
        QVariant::new()
    }
}

#[cfg(test)]
mod tests {}
