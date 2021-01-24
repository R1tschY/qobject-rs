use crate::core::thread::QThread;
use crate::core::{Connection, QMetaObject};
use crate::QBox;
use std::borrow::Cow;
use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_char;
use std::ptr;

cpp! {{
    #include <QObject>
}}

opaque_struct!(QObject);
impl_qobject_ref!(QObject);

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
        unsafe {
            QBox::from_raw(cpp!(unsafe [] -> *mut QObject as "QObject*" {
                return new QObject(nullptr);
            }))
        }
    }

    pub fn new_with_parent(parent: &mut QObject) -> *mut QObject {
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
            obj->deleteLater();
        })
    }

    unsafe fn meta_object(obj: &QObject) -> &'static QMetaObject {
        &*cpp!(unsafe [obj as "const QObject*"] -> *const QMetaObject as "const QMetaObject*" {
            return obj->metaObject();
        })
    }

    fn move_to_thread(obj: &mut QObject, target_thread: *mut QThread) {
        cpp!(unsafe [obj as "QObject*", target_thread as  "QThread*"] {
            obj->moveToThread(target_thread);
        });
    }

    fn connect_internal(
        sender: &QObject,
        signal: &CStr,
        receiver: &QObject,
        method: &CStr,
        type_: i32,
    ) -> Connection {
        let signal = signal.as_ptr();
        let method = method.as_ptr();
        cpp!(unsafe [sender as "const QObject*",
                     signal as "const char*",
                     receiver as "const QObject*",
                     method as "const char*",
                     type_ as "Qt::ConnectionType"] -> Connection as "QMetaObject::Connection" {
            return QObject::connect(sender, signal, receiver, method, type_);
        })
    }

    pub fn connect<R: Into<Signal>, S: Into<Slot>, T: Into<ConnectionType>>(
        sender: &QObject,
        signal: R,
        receiver: &QObject,
        method: S,
        type_: T,
    ) -> Connection {
        Self::connect_internal(
            sender,
            signal.into().as_cstr(),
            receiver,
            method.into().as_cstr(),
            type_.into().into(),
        )
    }

    fn disconnect_internal(
        sender: &QObject,
        signal: &CStr,
        receiver: &QObject,
        method: &CStr,
    ) -> bool {
        let signal = signal.as_ptr();
        let method = method.as_ptr();
        cpp!(unsafe [sender as "const QObject*",
                     signal as "const char*",
                     receiver as "const QObject*",
                     method as "const char*"] -> bool as "bool" {
            return QObject::disconnect(sender, signal, receiver, method);
        })
    }

    pub fn disconnect<R: Into<Signal>, S: Into<Slot>>(
        sender: &QObject,
        signal: R,
        receiver: &QObject,
        method: S,
    ) -> bool {
        Self::disconnect_internal(
            sender,
            signal.into().as_cstr(),
            receiver,
            method.into().as_cstr(),
        )
    }

    fn disconnect_from_internal(&self, receiver: &QObject, method: Option<&CStr>) -> bool {
        let method = method.map_or(ptr::null(), |p| p.as_ptr());
        cpp!(unsafe [self as "const QObject*",
                     receiver as "const QObject*",
                     method as "const char*"] -> bool as "bool" {
            return self->disconnect(receiver, method);
        })
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
        unsafe { QObject::inherits(self.as_qobject(), class_name.as_ptr()) }
    }

    unsafe fn delete_later(&mut self) {
        QObject::delete_later(self.as_qobject_mut())
    }

    fn meta_object(&self) -> &'static QMetaObject {
        unsafe { QObject::meta_object(self.as_qobject()) }
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
