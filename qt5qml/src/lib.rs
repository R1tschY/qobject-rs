#[macro_use]
extern crate cpp;

/// Define a opaque struct.
#[macro_export]
macro_rules! opaque_struct {
    ($x:ident) => {
        #[repr(C)]
        pub struct $x {
            _private: [u8; 0],
        }
    };
}

/// Create a `&'static CStr` from a string literal.
#[macro_export]
macro_rules! cstr {
    ($strlit:expr) => {
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($strlit, "\0").as_bytes()) }
    };
}

/// Get slot connect name.
///
/// Equivalent to Qt SLOT macro
///
/// ```rust
/// # use qt5qml::core::{QObject, ConnectionType};
/// # use qt5qml::{signal, slot};
/// # let object1 = QObject::new();
/// # let object2 = QObject::new();
///
/// QObject::connect(
///     &object1, signal!("triggered()"),
///     &object2, slot!("onTriggered()"),
///     ConnectionType::default());
/// ```
#[macro_export]
macro_rules! slot {
    ($strlit:expr) => {
        $crate::core::Slot::from_raw($crate::cstr!(concat!("1", $strlit)))
    };
}

/// Get signal connect name.
///
/// Equivalent to Qt SIGNAL macro.
///
/// ```rust
/// # use qt5qml::core::{QObject, ConnectionType};
/// # use qt5qml::{signal, slot};
/// # let object1 = QObject::new();
/// # let object2 = QObject::new();
///
/// QObject::connect(
///     &object1, signal!("triggered()"),
///     &object2, slot!("onTriggered()"),
///     ConnectionType::default());
/// ```
#[macro_export]
macro_rules! signal {
    ($strlit:expr) => {
        $crate::core::Signal::from_raw($crate::cstr!(concat!("2", $strlit)))
    };
}

macro_rules! impl_qobject_ref {
    ($ty:ty) => {
        impl crate::core::QObjectRef for $ty {
            fn as_qobject_mut(&mut self) -> &mut crate::core::QObject {
                unsafe { &mut *(self as *mut _ as *mut crate::core::QObject) }
            }

            fn as_qobject(&self) -> &crate::core::QObject {
                unsafe { &*(self as *const _ as *const crate::core::QObject) }
            }
        }
    };
}

#[macro_export]
macro_rules! new_qobject_helper {
    ($ty:ty, $cpp:expr) => {
        impl $ty {
            pub fn new(parent: Option<&mut QObject>) -> QBox<$ty> {
                let parent = parent.map_or(ptr::null_mut(), |p| p as *mut QObject);
                unsafe { QBox::from_raw($cpp) }
            }
        }
    };
}

pub mod core;
pub mod gui;
pub mod qml;

use crate::core::{QObject, QObjectRef};
use std::ops::{Deref, DerefMut};
use std::ptr;

pub trait Deletable {
    unsafe fn delete(&mut self);
}

pub struct CppBox<T: Deletable>(ptr::NonNull<T>);

impl<T: Deletable> CppBox<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(ptr::NonNull::new(ptr).expect("tried to created CppBox from null pointer"))
    }
}

impl<T: Deletable> Deref for CppBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Deletable> DerefMut for CppBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: Deletable> Drop for CppBox<T> {
    fn drop(&mut self) {
        unsafe { T::delete(self.0.as_mut()) };
    }
}

// QBox

/// Box for `QObject`s
pub struct QBox<T: QObjectRef>(ptr::NonNull<T>);

impl<T: QObjectRef> QBox<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(ptr::NonNull::new(ptr).expect("tried to create a QBox from a null pointer"))
    }
}

impl<T: QObjectRef> Deref for QBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: QObjectRef> DerefMut for QBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.0.as_mut() }
    }
}

impl<T: QObjectRef> Drop for QBox<T> {
    fn drop(&mut self) {
        unsafe { QObject::delete(self.deref_mut().as_qobject_mut()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_static_cstr() {
        let cstr: &CStr = cstr!("ABC");
        assert_eq!(cstr, CString::new("ABC").unwrap().as_c_str())
    }

    #[test]
    fn test_signal() {
        assert_eq!(
            signal!("signal()").as_cstr(),
            CString::new("2signal()").unwrap().as_c_str()
        )
    }

    #[test]
    fn test_slot() {
        assert_eq!(
            slot!("slot()").as_cstr(),
            CString::new("1slot()").unwrap().as_c_str()
        )
    }
}
