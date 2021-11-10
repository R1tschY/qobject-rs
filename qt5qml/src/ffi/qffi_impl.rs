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
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl QMetaMethod {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QMetaMethod_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QMetaMethod {
    #[inline]
    fn default() -> Self {
        QMetaMethod::new()
    }
}

impl Drop for QMetaMethod {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QMetaMethod_destroy(self) }
    }
}

impl PartialEq for QMetaMethod {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QMetaMethod_equals(self, other) }
    }
}
impl Eq for QMetaMethod { }

impl QMetaEnum {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QMetaEnum_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QMetaEnum {
    #[inline]
    fn default() -> Self {
        QMetaEnum::new()
    }
}

impl QMetaProperty {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QMetaProperty_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QMetaProperty {
    #[inline]
    fn default() -> Self {
        QMetaProperty::new()
    }
}

impl Drop for QMetaProperty {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QMetaProperty_destroy(self) }
    }
}

impl QMetaObjectConnection {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QMetaObjectConnection_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QMetaObjectConnection {
    #[inline]
    fn default() -> Self {
        QMetaObjectConnection::new()
    }
}

impl Clone for QMetaObjectConnection {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QMetaObjectConnection_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl QVariant {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QVariant_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QVariant {
    #[inline]
    fn default() -> Self {
        QVariant::new()
    }
}

impl Clone for QVariant {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QVariant_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QVariant {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QVariant_equals(self, other) }
    }
}
impl Eq for QVariant { }

impl PartialOrd for QVariant {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QVariant {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { qffi_QVariant_cmp(self, other) } {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => unreachable!(),
        }
    }
}

impl Drop for QTimer {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl Drop for QCoreApplication {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl Drop for QGuiApplication {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
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
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl Drop for QQmlEngine {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl Drop for QQmlApplicationEngine {
    #[inline]
    fn drop(&mut self) {
        unsafe { qffi_QObject_destroy(self as *mut _ as *mut crate::ffi::QObject) }
    }
}

impl QObjectList {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QObjectList_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QObjectList {
    #[inline]
    fn default() -> Self {
        QObjectList::new()
    }
}

impl Clone for QObjectList {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QObjectList_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QObjectList {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QObjectList_equals(self, other) }
    }
}
impl Eq for QObjectList { }

impl QStringList {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QStringList_init(ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}
impl Default for QStringList {
    #[inline]
    fn default() -> Self {
        QStringList::new()
    }
}

impl Clone for QStringList {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            qffi_QStringList_clone(self, ret.as_mut_ptr());
            ret.assume_init()
        }
    }
}

impl PartialEq for QStringList {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { qffi_QStringList_equals(self, other) }
    }
}
impl Eq for QStringList { }
