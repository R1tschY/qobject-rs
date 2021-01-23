#![allow(unused)]

use qt5qml::core::{QMetaObject, QMetaProperty, QObject, QObjectRef, QString, ToQString};
use qt5qml::cstr;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    _qobject: *mut TestObject,
    slot_calls: i32,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self {
            _qobject: qobject,
            slot_calls: 0,
        }
    }

    pub fn slot_calls(&self) -> i32 {
        self.slot_calls
    }

    pub fn slot(&mut self) {
        self.slot_calls += 1;
    }

    pub fn slot_with_args(&mut self, _arg: &QString) {
        self.slot_calls += 1;
    }

    pub fn echo_slot(&mut self, arg: &QString) -> QString {
        self.slot_calls += 1;
        QString::clone(arg)
    }
}

fn get_props(obj: &QMetaObject) -> HashMap<String, QMetaProperty> {
    obj.own_properties()
        .map(|e| (e.name().to_str().unwrap().to_owned(), e))
        .collect()
}

fn slot_calls(obj: &mut QObject) -> i32 {
    let props = get_props(obj.meta_object());
    props
        .get("slotCalls")
        .unwrap()
        .read(obj.as_qobject())
        .try_into()
        .unwrap()
}

#[test]
fn check_invoke_method() {
    unsafe {
        let mut object = TestObject::new(ptr::null_mut());
        let success =
            QMetaObject::build_invoke_method(object.as_qobject_mut(), cstr!("slot")).invoke();
        assert!(success);
        assert_eq!(1, slot_calls(object.as_qobject_mut()));
    }
}

#[test]
fn check_nonexisting_method() {
    unsafe {
        let mut object = TestObject::new(ptr::null_mut());
        let success =
            QMetaObject::build_invoke_method(object.as_qobject_mut(), cstr!("nonexisting"))
                .invoke();
        assert!(!success);
    }
}

#[test]
fn check_nonargs_method() {
    unsafe {
        let mut object = TestObject::new(ptr::null_mut());
        let success = QMetaObject::build_invoke_method(object.as_qobject_mut(), cstr!("slot"))
            .arg::<QString>(&QString::new())
            .invoke();
        assert!(!success);
        assert_eq!(0, slot_calls(object.as_qobject_mut()));
    }
}

#[test]
fn check_invoke_method_rtype() {
    unsafe {
        let mut object = TestObject::new(ptr::null_mut());
        let mut ret: QString = QString::new();
        let success = QMetaObject::build_invoke_method(object.as_qobject_mut(), cstr!("echoSlot"))
            .arg::<QString>(&"<!>".to_qstring())
            .ret::<QString>(&mut ret)
            .invoke();
        assert!(success);
        assert_eq!(1, slot_calls(object.as_qobject_mut()));
        assert_eq!("<!>", ret.to_string());
    }
}
