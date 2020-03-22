use qt5qml::core::{QMetaProperty, QObjectRef};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    qobject: *mut qt5qml::core::QObject,
    mydata: String,
    prop_rw: String,
    prop_w: String,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut qt5qml::core::QObject) -> Self {
        Self {
            qobject,
            mydata: "".into(),
            prop_rw: "".to_string(),
            prop_w: "".to_string(),
        }
    }

    pub fn prop_r(&self) -> qt5qml::core::QString {
        "Hello Qt!".into()
    }

    pub fn prop_rw(&self) -> qt5qml::core::QString {
        (&self.prop_rw).into()
    }

    pub fn set_prop_rw(&mut self, value: &qt5qml::core::QString) {
        self.prop_rw = value.into();
    }

    pub fn set_prop_w(&mut self, value: &qt5qml::core::QString) {
        self.prop_w = value.into();
    }
}

#[test]
fn test_meta_object() {
    let obj = TestObject::new(ptr::null_mut());
    assert!(obj.inherits(&CString::new("QObject").unwrap()));
    assert!(obj.inherits(&CString::new("TestObject").unwrap()));
    assert!(!obj.inherits(&CString::new("QAbstractListModel").unwrap()));

    let meta = obj.meta_object();
    assert_eq!(
        CString::new("TestObject").unwrap().as_c_str(),
        meta.class_name()
    );

    let props: HashMap<String, QMetaProperty> = meta
        .own_properties()
        .map(|e| (e.name().to_str().unwrap().to_owned(), e))
        .collect();

    assert!(props.contains_key("prop_rw"));
    assert!(props.contains_key("prop_r"));
    assert!(props.contains_key("prop_w"));
}
