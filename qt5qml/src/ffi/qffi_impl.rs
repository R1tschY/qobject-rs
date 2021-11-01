use std::mem::MaybeUninit;
use std::cmp::Ordering;

use super::qffi::*;


impl QString {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QString_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QString {
    #[inline]
    fn default() -> Self {
        QString::new()
    }
}

impl Drop for QString {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QString_destroy(self) }
    }
}

impl Clone for QString {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QString_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QString_equals(self, other) }
    }
}
impl Eq for QString { }

impl QByteArray {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QByteArray_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QByteArray {
    #[inline]
    fn default() -> Self {
        QByteArray::new()
    }
}

impl Drop for QByteArray {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QByteArray_destroy(self) }
    }
}

impl Clone for QByteArray {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QByteArray_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QByteArray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QByteArray_equals(self, other) }
    }
}
impl Eq for QByteArray { }

impl QUrl {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QUrl_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QUrl {
    #[inline]
    fn default() -> Self {
        QUrl::new()
    }
}

impl Drop for QUrl {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QUrl_destroy(self) }
    }
}

impl Clone for QUrl {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QUrl_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QUrl {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QUrl_equals(self, other) }
    }
}
impl Eq for QUrl { }

impl PartialOrd for QUrl {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QUrl {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { qffi_QUrl_cmp(self, other) } {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl Drop for QObject {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self) }
    }
}

impl Drop for QTimer {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::core::QObject) }
    }
}

impl Drop for QCoreApplication {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::core::QObject) }
    }
}

impl Drop for QGuiApplication {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::core::QObject) }
    }
}

impl QHashIntQByteArray {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QHashIntQByteArray_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QHashIntQByteArray {
    #[inline]
    fn default() -> Self {
        QHashIntQByteArray::new()
    }
}

impl Drop for QHashIntQByteArray {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QHashIntQByteArray_destroy(self) }
    }
}

impl Clone for QHashIntQByteArray {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QHashIntQByteArray_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QHashIntQByteArray {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QHashIntQByteArray_equals(self, other) }
    }
}
impl Eq for QHashIntQByteArray { }

impl Drop for QThread {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::core::QObject) }
    }
}
