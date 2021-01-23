#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    _qobject: *mut TestObject,
    prop_rw: i32,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self {
            _qobject: qobject,
            prop_rw: 42,
        }
    }

    pub fn prop_r(&self) -> qt5qml::core::QString {
        "Hello Qt!".into()
    }

    pub fn prop_rw(&self) -> i32 {
        self.prop_rw
    }

    pub fn set_prop_rw(&mut self, value: i32) {
        self.prop_rw = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;
    use qt5qml::core::{QMetaObject, QMetaProperty, QObjectRef, QVariant};
    use std::collections::HashMap;
    use std::ffi::CString;

    fn get_props(obj: &QMetaObject) -> HashMap<String, QMetaProperty> {
        obj.own_properties()
            .map(|e| (e.name().to_str().unwrap().to_owned(), e))
            .collect()
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

        let props = get_props(meta);
        assert!(props.contains_key("prop_rw"));
        assert!(props.contains_key("prop_r"));
    }

    #[test]
    fn read_prop() {
        let obj = TestObject::new(ptr::null_mut());
        let props = get_props(obj.meta_object());

        let value = props.get("prop_r").unwrap().read(obj.as_qobject());
        assert_eq!(value, (QVariant::from("Hello Qt!")));
    }

    #[test]
    fn written_value_can_be_read() {
        let mut obj = TestObject::new(ptr::null_mut());
        let props = get_props(obj.meta_object());

        assert!(props
            .get("prop_rw")
            .unwrap()
            .write(obj.as_qobject_mut(), &5.into()));
        assert_eq!(
            props.get("prop_rw").unwrap().read(obj.as_qobject()),
            5.into()
        );
    }

    #[test]
    fn read_only_prop_not_writeable() {
        let mut obj = TestObject::new(ptr::null_mut());
        let props = get_props(obj.meta_object());

        assert!(!props
            .get("prop_r")
            .unwrap()
            .write(obj.as_qobject_mut(), &"test".into()));
    }

    #[test]
    fn write_with_wrong_type_not_accepted() {
        let mut obj = TestObject::new(ptr::null_mut());
        let props = get_props(obj.meta_object());
        let value: QVariant = "Hello".into();
        dbg!(&value);

        assert!(!props
            .get("prop_rw")
            .unwrap()
            .write(obj.as_qobject_mut(), &value));
    }
}
