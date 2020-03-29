use crate::core::{QObject, QVariant};
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;

cpp! {{
    #include <QMetaObject>
    #include <QMetaProperty>
}}

opaque_struct!(QMetaObject);

cpp_class!(
    #[derive(Clone)]
    pub unsafe struct Connection as "QMetaObject::Connection"
);

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
        cpp!(unsafe [self as "const QMetaObject*"] -> i32 as "int" {
            return self->propertyCount();
        })
    }

    pub fn property_offset(&self) -> i32 {
        cpp!(unsafe [self as "const QMetaObject*"] -> i32 as "int" {
            return self->propertyOffset();
        })
    }

    pub fn property(&self, index: i32) -> QMetaProperty {
        cpp!(unsafe [self as "const QMetaObject*", index as "int"] -> QMetaProperty as "QMetaProperty" {
            return self->property(index);
        })
    }

    pub fn own_properties(&self) -> PropertyIterator {
        PropertyIterator {
            obj: self,
            index: self.property_offset(),
            count: self.property_count(),
        }
    }
}

impl Connection {
    /// Connection was successful established
    pub fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "const QMetaObject::Connection*"] -> bool as "bool" {
            return *self;
        })
    }

    /// Calls QObject::disconnect with connection
    pub fn disconnect(&self) -> bool {
        cpp!(unsafe [self as "const QMetaObject::Connection*"] -> bool as "bool" {
            return QObject::disconnect(*self);
        })
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

cpp_class!(#[derive(Clone)] pub unsafe struct QMetaProperty as "QMetaProperty");

impl QMetaProperty {
    pub fn has_notify_signal(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->hasNotifySignal();
        })
    }

    pub fn is_constant(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isConstant();
        })
    }

    pub fn is_designable(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isDesignable();
        })
    }

    pub fn is_enum_type(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isEnumType();
        })
    }

    pub fn is_final(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isFinal();
        })
    }

    pub fn is_flag_type(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isFlagType();
        })
    }

    pub fn is_readable(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isReadable();
        })
    }

    pub fn is_resettable(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isResettable();
        })
    }

    pub fn is_scriptable(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isScriptable();
        })
    }

    pub fn is_stored(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isStored();
        })
    }

    pub fn is_user(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isUser();
        })
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.is_readable()
    }

    pub fn is_writable(&self) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*"] -> bool as "bool" {
            return self->isWritable();
        })
    }

    pub fn name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const QMetaProperty*"] -> *const c_char as "const char*" {
                    return self->name();
                }),
            )
        }
    }

    pub fn notify_signal(&self) -> QMetaMethod {
        cpp!(unsafe [self as "const QMetaProperty*"] -> QMetaMethod as "QMetaMethod" {
            return self->notifySignal();
        })
    }

    pub fn notify_signal_index(&self) -> i32 {
        cpp!(unsafe [self as "const QMetaProperty*"] -> i32 as "int" {
            return self->notifySignalIndex();
        })
    }

    pub fn property_index(&self) -> i32 {
        cpp!(unsafe [self as "const QMetaProperty*"] -> i32 as "int" {
            return self->propertyIndex();
        })
    }

    pub fn read(&self, object: &QObject) -> QVariant {
        cpp!(unsafe [self as "const QMetaProperty*",
                     object as "const QObject*"] -> QVariant as "QVariant" {
            return self->read(object);
        })
    }

    pub unsafe fn read_on_gadget(&self, gadget: &c_void) -> QVariant {
        cpp!(unsafe [self as "const QMetaProperty*",
                     gadget as "const void*"] -> QVariant as "QVariant" {
            return self->readOnGadget(gadget);
        })
    }

    pub fn reset(&self, object: &mut QObject) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*", object as "QObject*"] -> bool as "bool" {
            return self->reset(object);
        })
    }

    pub unsafe fn reset_on_gadget(&self, gadget: &mut c_void) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*", gadget as "void*"] -> bool as "bool" {
            return self->resetOnGadget(gadget);
        })
    }

    pub fn revision(&self) -> i32 {
        cpp!(unsafe [self as "const QMetaProperty*"] -> i32 as "int" {
            return self->revision();
        })
    }

    pub fn type_name(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(
                cpp!([self as "const QMetaProperty*"] -> *const c_char as "const char*" {
                    return self->typeName();
                }),
            )
        }
    }

    pub fn write(&self, object: &mut QObject, value: &QVariant) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*",
                     object as "QObject*",
                     value as "const QVariant*"] -> bool as "bool" {
            return self->write(object, *value);
        })
    }

    pub unsafe fn write_on_gadget(&self, gadget: &mut c_void, value: &QVariant) -> bool {
        cpp!(unsafe [self as "const QMetaProperty*",
                     gadget as "void*",
                     value as "const QVariant*"] -> bool as "bool" {
            return self->writeOnGadget(gadget, *value);
        })
    }
}

cpp_class!(#[derive(Clone)] pub unsafe struct QMetaMethod as "QMetaMethod");
