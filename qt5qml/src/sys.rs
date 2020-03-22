use crate::QBox;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

macro_rules! opaque_struct {
    ($x:ident) => {
        #[repr(C)]
        pub struct $x {
            _private: [u8; 0],
        }
    };
}

cpp! {{
    #include <QObject>
    #include <QMetaObject>
    #include <QMetaProperty>
}}

opaque_struct!(QObject);
opaque_struct!(QMetaObject);
cpp_class!(pub unsafe struct QMetaProperty as "QMetaProperty");

impl QObject {
    pub unsafe fn new(parent: *mut QObject) -> *mut QObject {
        cpp!(unsafe [parent as "QObject*"] -> *mut QObject as "QObject*" {
            return new QObject(parent);
        })
    }

    pub unsafe fn inherits(obj: *const QObject, class_name: *const c_char) -> bool {
        cpp!(unsafe [obj as "const QObject*", class_name as "const char*"] -> bool as "bool" {
            return obj->inherits(class_name);
        })
    }

    pub unsafe fn delete(obj: *mut QObject) {
        cpp!(unsafe [obj as "QObject*"] {
            delete obj;
        })
    }

    pub unsafe fn delete_later(obj: *mut QObject) {
        cpp!(unsafe [obj as "QObject*"] {
            delete obj;
        })
    }

    unsafe fn meta_object(obj: *const QObject) -> &'static QMetaObject {
        &*cpp!(unsafe [obj as "const QObject*"] -> *const QMetaObject as "const QMetaObject*" {
            return obj->metaObject();
        })
    }
}

pub trait QObjectLike {
    fn get_qobject_mut_ptr(&mut self) -> *mut QObject;
    fn get_qobject_ptr(&self) -> *const QObject;

    fn inherits(&self, class_name: &str) -> bool {
        let class_name = CString::new(class_name).expect("NUL in string");
        unsafe { QObject::inherits(self.get_qobject_ptr(), class_name.as_ptr()) }
    }

    unsafe fn delete(&mut self) {
        QObject::delete(self.get_qobject_mut_ptr())
    }

    unsafe fn delete_later(&mut self) {
        QObject::delete_later(self.get_qobject_mut_ptr())
    }

    fn meta_object(&self) -> &QMetaObject {
        unsafe { QObject::meta_object(self.get_qobject_ptr()) }
    }
}

impl QMetaObject {
    pub fn class_name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const QMetaObject*"] -> *const c_char as "const char*" {
                    return self->className();
                }),
            )
        }
    }

    pub fn property_count(&self) -> i32 {
        unsafe {
            cpp!([self as "const QMetaObject*"] -> i32 as "int" {
                return self->propertyCount();
            })
        }
    }

    pub fn property_offset(&self) -> i32 {
        unsafe {
            cpp!([self as "const QMetaObject*"] -> i32 as "int" {
                return self->propertyOffset();
            })
        }
    }

    pub fn property(&self, index: i32) -> QMetaProperty {
        unsafe {
            cpp!([self as "const QMetaObject*", index as "int"] -> QMetaProperty as "QMetaProperty" {
                return self->property(index);
            })
        }
    }

    pub fn own_properties(&self) -> PropertyIterator {
        PropertyIterator {
            obj: self,
            index: self.property_offset(),
            count: self.property_count(),
        }
    }
}

pub struct PropertyIterator<'t> {
    obj: &'t QMetaObject,
    index: i32,
    count: i32,
}

impl<'t> Iterator for PropertyIterator<'t> {
    type Item = QMetaProperty;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            self.index += 1;
            Some(self.obj.property(self.index - 1))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.count - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl QMetaProperty {
    pub fn name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const QMetaProperty*"] -> *const c_char as "const char*" {
                    return self->name();
                }),
            )
        }
    }
}
