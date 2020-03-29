use qt5qml::core::{QModelIndex, QVariant};
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

#[cfg(test)]
mod tests {}
