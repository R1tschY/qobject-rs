use qt5qml::core::{QMetaObject, QMetaProperty, QObjectRef, QVariant};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    qobject: *mut TestObject,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self { qobject }
    }

    pub unsafe fn emit_signal_0(&mut self) {
        (&mut *self.qobject).signal0();
    }
}
