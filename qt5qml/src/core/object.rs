use crate::core::QMetaObject;
use crate::QBox;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

cpp! {{
    #include <QObject>
}}

opaque_struct!(QObject);

impl QObject {
    pub unsafe fn new(parent: &mut QObject) -> *mut QObject {
        cpp!(unsafe [parent as "QObject*"] -> *mut QObject as "QObject*" {
            return new QObject(parent);
        })
    }

    pub unsafe fn inherits(obj: &QObject, class_name: *const c_char) -> bool {
        cpp!(unsafe [obj as "const QObject*", class_name as "const char*"] -> bool as "bool" {
            return obj->inherits(class_name);
        })
    }

    pub unsafe fn delete(obj: &mut QObject) {
        cpp!(unsafe [obj as "QObject*"] {
            delete obj;
        })
    }

    pub unsafe fn delete_later(obj: &mut QObject) {
        cpp!(unsafe [obj as "QObject*"] {
            delete obj;
        })
    }

    unsafe fn meta_object(obj: &QObject) -> &'static QMetaObject {
        &*cpp!(unsafe [obj as "const QObject*"] -> *const QMetaObject as "const QMetaObject*" {
            return obj->metaObject();
        })
    }
}

pub trait QObjectRef {
    fn get_qobject_mut(&mut self) -> &mut QObject;
    fn get_qobject(&self) -> &QObject;

    fn inherits(&self, class_name: &CStr) -> bool {
        unsafe { QObject::inherits(self.get_qobject(), class_name.as_ptr()) }
    }

    unsafe fn delete(&mut self) {
        QObject::delete(self.get_qobject_mut())
    }

    unsafe fn delete_later(&mut self) {
        QObject::delete_later(self.get_qobject_mut())
    }

    fn meta_object(&self) -> &'static QMetaObject {
        unsafe { QObject::meta_object(self.get_qobject()) }
    }
}
