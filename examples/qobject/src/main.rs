mod sys {
    include!(concat!(env!("OUT_DIR"), "/qffi_MyQObject.rs"));

    pub struct MyQObjectPrivate {
        qobject: *mut QObject,
        mydata: String,
    }

    impl MyQObjectPrivate {
        pub fn new(qobject: *mut QObject) -> Self {
            Self {
                qobject,
                mydata: "".into(),
            }
        }
    }
}

use crate::sys::MyQObject;
use qt5qml::core::QObjectRef;
use std::ffi::CString;
use std::ptr;

fn main() {
    let obj = MyQObject::new(ptr::null_mut());
    assert!(obj.inherits(&CString::new("QObject").unwrap()));
    assert!(obj.inherits(&CString::new("MyQObject").unwrap()));
    assert!(!obj.inherits(&CString::new("QAbstractListModel").unwrap()));

    let meta = obj.meta_object();
    println!("Class name: {:?}", meta.class_name());

    for prop in meta.own_properties() {
        println!("Prop name: {:?}", prop.name());
    }
}
