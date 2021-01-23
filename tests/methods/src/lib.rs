#![allow(unused)]

use qt5qml::core::{QMetaObject, QMetaProperty, QObject, QObjectRef, QString, ToQString};
use qt5qml::cstr;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::c_void;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    _qobject: *mut TestObject,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self { _qobject: qobject }
    }

    pub fn method_safe_return(&mut self) -> i32 {
        42
    }

    pub fn method_unsafe_return(&mut self) -> QString {
        "Hello".into()
    }

    pub fn method_with_args(&mut self, _value1: &QString, value2: u64) -> i32 {
        value2 as i32
    }

    pub fn custom_event(&mut self, _arg: *mut c_void) {
        unimplemented!()
    }

    pub fn sender(&self) -> *mut QObject {
        unsafe { &mut *self._qobject }.sender()
    }

    pub fn object_name(&self) -> QString {
        unsafe { &mut *self._qobject }.object_name()
    }

    pub fn dump_object_info(&self) {
        unsafe { &mut *self._qobject }.dump_object_info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_method_safe_return() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        assert_eq!(42, private.method_safe_return());
    }

    #[test]
    fn check_method_unsafe_return() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        assert_eq!("Hello", private.method_unsafe_return().to_string());
    }

    #[test]
    fn check_method_with_args() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        assert_eq!(89, private.method_with_args(&QString::new(), 89));
    }

    #[test]
    fn check_proxy_safe_return() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        assert_eq!(ptr::null_mut(), private.sender());
    }

    #[test]
    fn check_proxy_unsafe_return() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        assert_eq!("", &private.object_name().to_string());
    }

    #[test]
    fn check_proxy_void_return() {
        let mut object = TestObject::new(ptr::null_mut());
        let private = object.get_private();

        private.dump_object_info();
    }
}
