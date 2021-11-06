use crate::core::{ConnectionType, ConnectionTypeKind, QObject, QString, QVariant};
use crate::ffi::*;
use std::ffi::{c_void, CStr};
use std::mem::transmute;
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct QMetaObject(pub(crate) crate::ffi::QMetaObject);
impl_ffi_trait!(QMetaObject);

impl QMetaObject {
    pub fn class_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(qffi_QMetaObject_className(self.to_inner())) }
    }

    pub fn property_count(&self) -> i32 {
        unsafe { qffi_QMetaObject_propertyCount(self.to_inner()) }
    }

    pub fn property_offset(&self) -> i32 {
        unsafe { qffi_QMetaObject_propertyOffset(self.to_inner()) }
    }

    pub fn property(&self, index: i32) -> QMetaProperty {
        unsafe { QMetaProperty(qffi_QMetaObject_property(self.to_inner(), index)) }
    }

    pub fn properties(&self) -> PropertyIterator {
        PropertyIterator {
            obj: self,
            index: 0,
            count: self.property_count(),
        }
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
        member: &'a CStr,
    ) -> InvokeMethodBuilder<'a> {
        InvokeMethodBuilder::new(obj, member)
    }
}

#[repr(C)]
pub struct QMetaObjectConnection(pub(crate) crate::ffi::QMetaObjectConnection);
impl_ffi_trait!(QMetaObjectConnection);

impl QMetaObjectConnection {
    /// Connection was successful established
    pub fn is_valid(&self) -> bool {
        unsafe { qffi_QMetaObjectConnection_isValid(self.to_inner()) }
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

#[repr(C)]
pub struct QMetaProperty(pub(crate) crate::ffi::QMetaProperty);
impl_ffi_trait!(QMetaProperty);

impl QMetaProperty {
    #[inline]
    pub fn has_notify_signal(&self) -> bool {
        unsafe { qffi_QMetaProperty_hasNotifySignal(self.to_inner()) }
    }

    #[inline]
    pub fn is_constant(&self) -> bool {
        unsafe { qffi_QMetaProperty_isConstant(self.to_inner()) }
    }

    #[inline]
    pub fn is_designable(&self) -> bool {
        unsafe { qffi_QMetaProperty_isDesignable(self.to_inner()) }
    }

    #[inline]
    pub fn is_enum_type(&self) -> bool {
        unsafe { qffi_QMetaProperty_isEnumType(self.to_inner()) }
    }

    #[inline]
    pub fn is_final(&self) -> bool {
        unsafe { qffi_QMetaProperty_isFinal(self.to_inner()) }
    }

    #[inline]
    pub fn is_flag_type(&self) -> bool {
        unsafe { qffi_QMetaProperty_isFlagType(self.to_inner()) }
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        unsafe { qffi_QMetaProperty_isReadable(self.to_inner()) }
    }

    #[inline]
    pub fn is_resettable(&self) -> bool {
        unsafe { qffi_QMetaProperty_isResettable(self.to_inner()) }
    }

    #[inline]
    pub fn is_scriptable(&self) -> bool {
        unsafe { qffi_QMetaProperty_isScriptable(self.to_inner()) }
    }

    #[inline]
    pub fn is_stored(&self) -> bool {
        unsafe { qffi_QMetaProperty_isStored(self.to_inner()) }
    }

    #[inline]
    pub fn is_user(&self) -> bool {
        unsafe { qffi_QMetaProperty_isUser(self.to_inner()) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.is_readable()
    }

    #[inline]
    pub fn is_writable(&self) -> bool {
        unsafe { qffi_QMetaProperty_isWritable(self.to_inner()) }
    }

    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(qffi_QMetaProperty_name(self.to_inner())) }
    }

    #[inline]
    pub fn notify_signal(&self) -> QMetaMethod {
        unsafe { QMetaMethod(qffi_QMetaProperty_notifySignal(self.to_inner())) }
    }

    #[inline]
    pub fn notify_signal_index(&self) -> i32 {
        unsafe { qffi_QMetaProperty_notifySignalIndex(self.to_inner()) }
    }

    #[inline]
    pub fn property_index(&self) -> i32 {
        unsafe { qffi_QMetaProperty_propertyIndex(self.to_inner()) }
    }

    #[inline]
    pub fn read(&self, object: &QObject) -> QVariant {
        unsafe {
            QVariant(init_ffi_struct(|v| {
                qffi_QMetaProperty_read(self.to_inner(), object.to_inner(), v)
            }))
        }
    }

    #[inline]
    pub unsafe fn read_on_gadget(&self, gadget: &c_void) -> QVariant {
        unsafe {
            QVariant(init_ffi_struct(|v| {
                qffi_QMetaProperty_readOnGadget(self.to_inner(), gadget, v)
            }))
        }
    }

    #[inline]
    pub fn reset(&self, object: &mut QObject) -> bool {
        unsafe { qffi_QMetaProperty_reset(self.to_inner(), object.to_inner_mut()) }
    }

    #[inline]
    pub unsafe fn reset_on_gadget(&self, gadget: &mut c_void) -> bool {
        unsafe { qffi_QMetaProperty_resetOnGadget(self.to_inner(), gadget) }
    }

    #[inline]
    pub fn revision(&self) -> i32 {
        unsafe { qffi_QMetaProperty_revision(self.to_inner()) }
    }

    #[inline]
    pub fn type_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(qffi_QMetaProperty_typeName(self.to_inner())) }
    }

    #[inline]
    pub fn write(&self, object: &mut QObject, value: &QVariant) -> bool {
        unsafe {
            qffi_QMetaProperty_write(self.to_inner(), object.to_inner_mut(), value.to_inner())
        }
    }

    #[inline]
    pub unsafe fn write_on_gadget(&self, gadget: &mut c_void, value: &QVariant) -> bool {
        unsafe { qffi_QMetaProperty_writeOnGadget(self.to_inner(), gadget, value.to_inner()) }
    }
}

#[repr(C)]
pub struct QMetaMethod(pub(crate) crate::ffi::QMetaMethod);

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
    #[inline]
    pub fn new<T: QtMetaType>(data: &T) -> Self {
        Self::from_raw(T::name().as_ptr(), data as *const _ as *const c_void)
    }

    #[inline]
    pub fn new_mut<T: QtMetaType>(data: &mut T) -> Self {
        Self::from_raw(T::name().as_ptr(), data as *mut _ as *const c_void)
    }

    #[inline]
    pub unsafe fn new_unchecked<T>(ty: &'static CStr, data: &T) -> Self {
        Self::from_raw(ty.as_ptr(), data as *const _ as *const c_void)
    }

    #[inline]
    pub unsafe fn new_mut_unchecked<T>(ty: &'static CStr, data: &mut T) -> Self {
        Self::from_raw(ty.as_ptr(), data as *mut _ as *const c_void)
    }

    #[inline]
    pub fn from_raw(name: *const c_char, data: *const c_void) -> Self {
        Self { name, data }
    }
}

impl Default for QGenericArgument {
    #[inline]
    fn default() -> Self {
        Self::from_raw(ptr::null(), ptr::null())
    }
}

pub struct InvokeMethodBuilder<'a> {
    obj: &'a mut QObject,
    member: &'a CStr,
    ret: Option<QGenericArgument>,
    type_: ConnectionType,
    arg_len: usize,
    args: [QGenericArgument; 10],
}

impl<'a> InvokeMethodBuilder<'a> {
    #[inline]
    pub fn new(obj: &'a mut QObject, member: &'a CStr) -> Self {
        Self {
            obj,
            member,
            ret: None,
            type_: ConnectionTypeKind::Auto.into(),
            arg_len: 0,
            args: [QGenericArgument::default(); 10],
        }
    }

    #[inline]
    pub fn arg<T: QtMetaType>(&mut self, t: &T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new(t);
        self.arg_len += 1;
        self
    }

    #[inline]
    pub fn arg_mut<T: QtMetaType>(&mut self, t: &mut T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_mut(t);
        self.arg_len += 1;
        self
    }

    #[inline]
    pub unsafe fn arg_unchecked<T>(&mut self, ty: &'static CStr, data: &T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_unchecked(ty, data);
        self.arg_len += 1;
        self
    }

    #[inline]
    pub unsafe fn arg_mut_unchecked<T>(&mut self, ty: &'static CStr, data: &mut T) -> &mut Self {
        assert!(self.arg_len < 10);
        self.args[self.arg_len] = QGenericArgument::new_mut_unchecked(ty, data);
        self.arg_len += 1;
        self
    }

    #[inline]
    pub fn ret<T: QtMetaType>(&mut self, t: &mut T) -> &mut Self {
        self.ret = Some(QGenericArgument::new_mut(t));
        self
    }

    #[inline]
    pub unsafe fn ret_unchecked<T>(&mut self, ty: &'static CStr, t: &mut T) -> &mut Self {
        self.ret = Some(QGenericArgument::new_mut_unchecked(ty, t));
        self
    }

    #[inline]
    pub fn type_(&mut self, type_: impl Into<ConnectionType>) -> &mut Self {
        self.type_ = type_.into();
        self
    }

    pub unsafe fn invoke(&mut self) -> bool {
        if let Some(ref ret) = self.ret {
            unsafe {
                qffi_QMetaObject_invokeMethodAndReturn(
                    self.obj.to_inner_mut(),
                    self.member.as_ptr(),
                    self.type_.into(),
                    transmute(ret),
                    transmute(self.args.as_ptr()),
                )
            }
        } else {
            unsafe {
                qffi_QMetaObject_invokeMethod(
                    self.obj.to_inner_mut(),
                    self.member.as_ptr(),
                    self.type_.into(),
                    transmute(self.args.as_ptr()),
                )
            }
        }
    }
}
