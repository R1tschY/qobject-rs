use crate::core::thread::QThread;
use crate::core::{QMetaObject, QMetaObjectConnection};
use crate::ffi::{
    init_ffi_struct, qffi_QObject_connect, qffi_QObject_destroy, qffi_QObject_disconnect2,
    qffi_QObject_disconnect3, qffi_QObject_disconnectConnection, qffi_QObject_inherits,
    qffi_QObject_init, qffi_QObject_metaObject, qffi_QObject_moveToThread, QffiWrapper,
};
use crate::QBox;
use std::borrow::Cow;
use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct QObject(pub(crate) crate::ffi::QObject);
impl_ffi_trait!(QObject);

impl QObjectRef for QObject {
    fn as_qobject_mut(&mut self) -> &mut QObject {
        self
    }

    fn as_qobject(&self) -> &QObject {
        self
    }
}

/// See enum Qt::ConnectionType
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConnectionTypeKind {
    Auto,
    Direct,
    Queued,
    BlockingQueued,
}

/// See enum Qt::ConnectionType
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConnectionType {
    kind: ConnectionTypeKind,
    unique: bool,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Signal(Cow<'static, CStr>);

#[derive(Clone, Eq, PartialEq)]
pub struct Slot(Cow<'static, CStr>);

impl QObject {
    pub fn new() -> QBox<QObject> {
        unsafe { QBox::from_raw(std::mem::transmute(qffi_QObject_init(ptr::null_mut()))) }
    }

    pub fn new_with_parent(parent: &mut QObject) -> *mut QObject {
        unsafe { std::mem::transmute(crate::ffi::qffi_QObject_init(parent.to_inner_mut())) }
    }

    fn move_to_thread(&mut self, mut target_thread: *mut QThread) {
        unsafe {
            std::mem::transmute(qffi_QObject_moveToThread(
                self.to_inner_mut(),
                std::mem::transmute(target_thread),
            ))
        }
    }

    pub fn connect<R: Into<Signal>, S: Into<Slot>, T: Into<ConnectionType>>(
        sender: &QObject,
        signal: R,
        receiver: &QObject,
        method: S,
        type_: T,
    ) -> QMetaObjectConnection {
        sender.connect_internal(
            signal.into().as_cstr(),
            receiver,
            method.into().as_cstr(),
            type_.into().into(),
        )
    }

    fn connect_internal(
        &self,
        signal: &CStr,
        receiver: &QObject,
        method: &CStr,
        type_: i32,
    ) -> QMetaObjectConnection {
        QMetaObjectConnection(init_ffi_struct(|dest| unsafe {
            qffi_QObject_connect(
                self.to_inner(),
                signal.as_ptr(),
                receiver.to_inner(),
                method.as_ptr(),
                type_,
                dest,
            )
        }))
    }

    fn disconnect_internal(&self, signal: &CStr, receiver: &QObject, method: &CStr) -> bool {
        unsafe {
            qffi_QObject_disconnect3(
                self.to_inner(),
                signal.as_ptr(),
                receiver.to_inner(),
                method.as_ptr(),
            )
        }
    }

    fn disconnect_from_internal(&self, receiver: &QObject, method: Option<&CStr>) -> bool {
        let method = method.map_or(ptr::null(), |p| p.as_ptr());
        unsafe { qffi_QObject_disconnect2(self.to_inner(), receiver.to_inner(), method) }
    }

    pub fn disconnect(connection: &QMetaObjectConnection) -> bool {
        unsafe { qffi_QObject_disconnectConnection(connection.to_inner()) }
    }

    pub fn object_name_changed_signal() -> Signal {
        signal!("objectNameChanged(const QString&)")
    }

    pub fn destroyed_signal() -> Signal {
        signal!("destroyed(QObject*)")
    }

    pub fn delete_later_slot() -> Slot {
        slot!("deleteLater()")
    }
}

impl ConnectionType {
    pub fn new(kind: ConnectionTypeKind, unique: bool) -> Self {
        Self { kind, unique }
    }
}

impl From<i32> for ConnectionType {
    fn from(value: i32) -> Self {
        let unique = (value & 0x80) != 0;
        let kind = match value & 0x0F {
            0 => ConnectionTypeKind::Auto,
            1 => ConnectionTypeKind::Direct,
            2 => ConnectionTypeKind::Queued,
            3 => ConnectionTypeKind::BlockingQueued,
            _ => panic!("unknown Qt::ConnectionType"),
        };
        Self::new(kind, unique)
    }
}

impl From<ConnectionTypeKind> for ConnectionType {
    fn from(value: ConnectionTypeKind) -> Self {
        Self::new(value, false)
    }
}

impl From<ConnectionType> for i32 {
    fn from(value: ConnectionType) -> Self {
        let unique: i32 = if value.unique { 0x80 } else { 0 };
        let kind: i32 = match value.kind {
            ConnectionTypeKind::Auto => 0,
            ConnectionTypeKind::Direct => 1,
            ConnectionTypeKind::Queued => 2,
            ConnectionTypeKind::BlockingQueued => 3,
        };
        kind | unique
    }
}

impl Default for ConnectionType {
    fn default() -> Self {
        Self::new(ConnectionTypeKind::Auto, false)
    }
}

impl Signal {
    pub fn new(value: &str) -> Result<Self, NulError> {
        Ok(Self(CString::new(format!("2{}", value))?.into()))
    }

    /// Create from raw meta method name (Starts with `2`).
    pub fn from_raw<T: Into<Cow<'static, CStr>>>(value: T) -> Self {
        Self(value.into())
    }

    pub fn as_cstr(&self) -> &CStr {
        self.0.as_ref()
    }
}

impl Slot {
    pub fn new(value: &str) -> Result<Self, NulError> {
        Ok(Self(CString::new(format!("1{}", value))?.into()))
    }

    /// Create from raw meta method name (Starts with `1`).
    pub fn from_raw<T: Into<Cow<'static, CStr>>>(value: T) -> Self {
        Self(value.into())
    }

    pub fn as_cstr(&self) -> &CStr {
        self.0.as_ref()
    }
}

impl From<Signal> for Slot {
    fn from(value: Signal) -> Self {
        Slot::from_raw(value.0)
    }
}

pub trait QObjectRef {
    fn as_qobject_mut(&mut self) -> &mut QObject;
    fn as_qobject(&self) -> &QObject;

    fn inherits(&self, class_name: &CStr) -> bool {
        unsafe { qffi_QObject_inherits(self.as_qobject().to_inner(), class_name.as_ptr()) }
    }

    unsafe fn delete(&mut self) {
        unsafe { qffi_QObject_destroy(self.as_qobject_mut().to_inner_mut()) }
    }

    unsafe fn delete_later(&mut self) {
        unsafe { qffi_QObject_destroy(self.as_qobject_mut().to_inner_mut()) }
    }

    fn meta_object(&self) -> &'static QMetaObject {
        unsafe { std::mem::transmute(qffi_QObject_metaObject(self.as_qobject().to_inner())) }
    }

    fn connect<R: Into<Signal>, S: Into<Slot>, T: Into<ConnectionType>>(
        &self,
        signal: R,
        receiver: &QObject,
        method: S,
        type_: T,
    ) -> QMetaObjectConnection {
        self.as_qobject().connect_internal(
            signal.into().as_cstr(),
            receiver,
            method.into().as_cstr(),
            type_.into().into(),
        )
    }

    fn disconnect<R: Into<Signal>, S: Into<Slot>>(
        &self,
        signal: R,
        receiver: &QObject,
        method: S,
    ) -> bool {
        self.as_qobject().disconnect_internal(
            signal.into().as_cstr(),
            receiver,
            method.into().as_cstr(),
        )
    }

    fn disconnect_from(&self, receiver: &QObject) -> bool {
        self.as_qobject().disconnect_from_internal(receiver, None)
    }

    fn disconnect_from_method<T: Into<Slot>>(&self, receiver: &QObject, method: T) -> bool {
        self.as_qobject()
            .disconnect_from_internal(receiver, Some(method.into().as_cstr()))
    }

    fn move_to_thread(&mut self, target_thread: Option<&mut QThread>) {
        QObject::move_to_thread(
            self.as_qobject_mut(),
            if let Some(t) = target_thread {
                t
            } else {
                ptr::null_mut()
            },
        );
    }
}
