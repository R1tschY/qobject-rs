use std::ffi::CString;

use qt5qml::core::QObjectRef;

include!(concat!(env!("OUT_DIR"), "/qffi_MyQObject.rs"));

pub struct MyQObjectPrivate {
    _qobject: *mut MyQObject,
}

impl MyQObjectPrivate {
    pub fn new(qobject: *mut MyQObject) -> Self {
        Self { _qobject: qobject }
    }
}

fn main() {
    let obj = MyQObject::new();
    assert!(obj.inherits(&CString::new("QObject").unwrap()));
    assert!(obj.inherits(&CString::new("MyQObject").unwrap()));
    assert!(!obj.inherits(&CString::new("QAbstractListModel").unwrap()));

    let meta = obj.meta_object();
    println!("Class name: {:?}", meta.class_name());

    for prop in meta.own_properties() {
        println!("Prop name: {:?}", prop.name());
    }
}
