mod sys {
    use qt5qml::{Deletable, QBox, QObjectLike};
    use std::ptr;
    include!(concat!(env!("OUT_DIR"), "/qffi_MyQObject.rs"));

    #[repr(C)]
    pub struct MyQObject {
        _private: [u8; 0],
    }

    impl MyQObject {
        pub fn new(parent: *mut QObject) -> QBox<MyQObject> {
            unsafe { QBox::from_raw(Qffi_MyQObject_new(parent) as *mut _ as *mut MyQObject) }
        }
    }

    impl Deletable for MyQObject {
        unsafe fn delete(&mut self) {
            QObject::delete(self.get_qobject_mut_ptr());
        }
    }

    impl QObjectLike for MyQObject {
        fn get_qobject_mut_ptr(&mut self) -> *mut QObject {
            self as *mut _ as *mut QObject
        }

        fn get_qobject_ptr(&self) -> *const QObject {
            self as *const _ as *const QObject
        }
    }

    // impl Default for *mut MyQObject {
    //     fn default() -> Self {
    //         Self::new(ptr::null_mut())
    //     }
    // }

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
use qt5qml::{QObject, QObjectLike};
use std::ffi::{c_void, CString};
use std::ptr;

fn main() {
    let obj = MyQObject::new(ptr::null_mut());
    assert!(obj.inherits("QObject"));
    assert!(obj.inherits("MyQObject"));
    assert!(!obj.inherits("QAbstractListModel"));

    let meta = obj.meta_object();
    println!("Class name: {:?}", meta.class_name());

    for prop in meta.own_properties() {
        println!("Prop name: {:?}", prop.name());
    }
}
