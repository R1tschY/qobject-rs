use crate::core::{ConnectionType, ConnectionTypeKind, QObject, QString, QVariant};
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::ptr;

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

    pub fn build_invoke_method<'a>(
        obj: &'a mut QObject,
        member: &'static CStr,
    ) -> InvokeMethodBuilder<'a, 'static> {
        InvokeMethodBuilder::new(obj, member)
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

pub trait QtMetaType {
    fn name() -> &'static CStr;
}

impl QtMetaType for QString {
    fn name() -> &'static CStr {
        cstr!("QString")
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct QGenericArgument {
    data: *const c_void,
    name: *const c_char,
}

impl QGenericArgument {
    pub fn new<T: QtMetaType>(data: &T) -> Self {
        Self::from_raw(T::name().as_ptr(), data as *const _ as *const c_void)
    }

    pub fn new_mut<T: QtMetaType>(data: &mut T) -> Self {
        Self::from_raw(T::name().as_ptr(), data as *mut _ as *const c_void)
    }

    pub unsafe fn new_unchecked<T>(ty: &'static CStr, data: &T) -> Self {
        Self::from_raw(ty.as_ptr(), data as *const _ as *const c_void)
    }

    pub unsafe fn new_mut_unchecked<T>(ty: &'static CStr, data: &mut T) -> Self {
        Self::from_raw(ty.as_ptr(), data as *mut _ as *const c_void)
    }

    pub fn from_raw(name: *const c_char, data: *const c_void) -> Self {
        Self { name, data }
    }
}

impl Default for QGenericArgument {
    fn default() -> Self {
        Self::from_raw(ptr::null(), ptr::null())
    }
}

pub struct InvokeMethodBuilder<'a, 'b> {
    obj: &'a mut QObject,
    member: &'b CStr,
    ret: Option<QGenericArgument>,
    type_: ConnectionType,
    arg_len: usize,
    args: [QGenericArgument; 10],
}

impl<'a, 'b> InvokeMethodBuilder<'a, 'b> {
    pub fn new(obj: &'a mut QObject, member: &'b CStr) -> Self {
        Self {
            obj,
            member,
            ret: None,
            type_: ConnectionTypeKind::Auto.into(),
            arg_len: 0,
            args: [QGenericArgument::default(); 10],
        }
    }

    pub fn arg<T: QtMetaType>(&mut self, t: &T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new(t);
        self.arg_len += 1;
        self
    }

    pub fn arg_mut<T: QtMetaType>(&mut self, t: &mut T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_mut(t);
        self.arg_len += 1;
        self
    }

    pub unsafe fn arg_unchecked<T>(&mut self, ty: &'static CStr, data: &T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_unchecked(ty, data);
        self.arg_len += 1;
        self
    }

    pub unsafe fn arg_mut_unchecked<T>(&mut self, ty: &'static CStr, data: &mut T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_mut_unchecked(ty, data);
        self.arg_len += 1;
        self
    }

    pub fn ret<T: QtMetaType>(&mut self, t: &mut T) -> &mut Self {
        self.ret = Some(QGenericArgument::new_mut(t));
        self
    }

    pub unsafe fn ret_unchecked<T>(&mut self, ty: &'static CStr, t: &mut T) -> &mut Self {
        self.ret = Some(QGenericArgument::new_mut_unchecked(ty, t));
        self
    }

    pub fn type_(&mut self, type_: impl Into<ConnectionType>) -> &mut Self {
        self.type_ = type_.into();
        self
    }

    pub unsafe fn invoke(&mut self) -> bool {
        let obj = self.obj as *mut QObject;
        let member = self.member.as_ptr();
        let args = self.args.as_ptr();
        let ty: i32 = self.type_.into();

        if let Some(ref ret) = self.ret {
            cpp!([
                obj as "QObject*",
                member as "const char *",
                ty as "qint32",
                ret as "const QGenericReturnArgument*",
                args as "const QGenericArgument*"
            ] -> bool as "bool" {
                return QMetaObject::invokeMethod(
                    obj, member, Qt::ConnectionType(ty), *ret, args[0], args[1], args[2], args[3], args[4], args[5],
                    args[6], args[7], args[8], args[9]);
            })
        } else {
            cpp!([
                obj as "QObject*",
                member as "const char *",
                ty as "qint32",
                args as "const QGenericArgument*"
            ] -> bool as "bool" {
                return QMetaObject::invokeMethod(
                    obj, member, Qt::ConnectionType(ty), args[0], args[1], args[2], args[3], args[4], args[5], args[6],
                    args[7], args[8], args[9]);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {}
}
