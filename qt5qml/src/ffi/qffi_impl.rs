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



impl Drop for QTimer {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QTimer_destroy(self) }
    }
}





