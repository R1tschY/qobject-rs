use crate::QBox;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

cpp! {{
    #include <QMetaObject>
    #include <QMetaProperty>
}}

opaque_struct!(QMetaObject);
cpp_class!(#[derive(Clone)] pub unsafe struct QMetaProperty as "QMetaProperty");

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
