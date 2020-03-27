use std::ffi::CString;
use std::ptr;

use qt5qml::core::QObject;
use qt5qml::core::QObjectRef;

include!(concat!(env!("OUT_DIR"), "/qffi_MyQObject.rs"));

pub struct MyQObjectPrivate {
    qobject: *mut MyQObject,
    mydata: String,
}

impl MyQObjectPrivate {
    pub fn new(qobject: *mut MyQObject) -> Self {
        Self {
            qobject,
            mydata: "".into(),
        }
    }
}

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
