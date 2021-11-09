use std::mem::transmute;
use std::iter::FromIterator;
use std::os::raw::c_int;

use crate::ffi::QffiWrapper;

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QObjectList(crate::ffi::QObjectList);
impl_ffi_trait!(QObjectList);

impl QObjectList {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> i32 {
        unsafe { crate::ffi::qffi_QObjectList_size(self.to_inner()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[*mut crate::core::QObject] {
        unsafe {
            let mut size: c_int = 0;
            let ptr = transmute(crate::ffi::qffi_QObjectList_asSlice(self.to_inner(), &mut size));
            if size != 0 {
                std::slice::from_raw_parts(ptr, size as usize)
            } else {
                &[]
            }
        }
    }

    pub fn push(&mut self, item: &*mut crate::core::QObject) {
        unsafe { crate::ffi::qffi_QObjectList_append(self.to_inner_mut(), transmute(item)) }
    }

    pub fn append(&mut self, value: &QObjectList) {
        unsafe { crate::ffi::qffi_QObjectList_appendList(self.to_inner_mut(), transmute(value)) }
    }

    pub fn extend_from_slice(&mut self, slice: &[*mut crate::core::QObject]) {
        unsafe { crate::ffi::qffi_QObjectList_appendSlice(self.to_inner_mut(), transmute(slice.as_ptr()), slice.len() as c_int) }
    }

    pub fn reserve(&mut self, additional: usize) {
        unsafe { crate::ffi::qffi_QObjectList_reserveAdditional(self.to_inner_mut(), additional as i32) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &*mut crate::core::QObject> {
        self.as_slice().iter()
    }
}

impl Extend<*mut crate::core::QObject> for QObjectList {
    fn extend<T: IntoIterator<Item = *mut crate::core::QObject>>(&mut self, iter: T) {
        for item in iter {
            self.push(&item);
        }
    }
}

impl<'a> Extend<&'a *mut crate::core::QObject> for QObjectList {
    fn extend<T: IntoIterator<Item = &'a *mut crate::core::QObject>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl FromIterator<*mut crate::core::QObject> for QObjectList {
    fn from_iter<T: IntoIterator<Item = *mut crate::core::QObject>>(iter: T) -> Self {
        let mut res = Self::default();
        res.extend(iter);
        res
    }
}

impl<'a> IntoIterator for &'a QObjectList {
    type Item = &'a *mut crate::core::QObject;
    type IntoIter = std::slice::Iter<'a, *mut crate::core::QObject>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QStringList(crate::ffi::QStringList);
impl_ffi_trait!(QStringList);

impl QStringList {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> i32 {
        unsafe { crate::ffi::qffi_QStringList_size(self.to_inner()) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[crate::core::QString] {
        unsafe {
            let mut size: c_int = 0;
            let ptr = transmute(crate::ffi::qffi_QStringList_asSlice(self.to_inner(), &mut size));
            if size != 0 {
                std::slice::from_raw_parts(ptr, size as usize)
            } else {
                &[]
            }
        }
    }

    pub fn push(&mut self, item: &crate::core::QString) {
        unsafe { crate::ffi::qffi_QStringList_append(self.to_inner_mut(), transmute(item)) }
    }

    pub fn append(&mut self, value: &QStringList) {
        unsafe { crate::ffi::qffi_QStringList_appendList(self.to_inner_mut(), transmute(value)) }
    }

    pub fn extend_from_slice(&mut self, slice: &[crate::core::QString]) {
        unsafe { crate::ffi::qffi_QStringList_appendSlice(self.to_inner_mut(), transmute(slice.as_ptr()), slice.len() as c_int) }
    }

    pub fn reserve(&mut self, additional: usize) {
        unsafe { crate::ffi::qffi_QStringList_reserveAdditional(self.to_inner_mut(), additional as i32) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &crate::core::QString> {
        self.as_slice().iter()
    }
}

impl Extend<crate::core::QString> for QStringList {
    fn extend<T: IntoIterator<Item = crate::core::QString>>(&mut self, iter: T) {
        for item in iter {
            self.push(&item);
        }
    }
}

impl<'a> Extend<&'a crate::core::QString> for QStringList {
    fn extend<T: IntoIterator<Item = &'a crate::core::QString>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl FromIterator<crate::core::QString> for QStringList {
    fn from_iter<T: IntoIterator<Item = crate::core::QString>>(iter: T) -> Self {
        let mut res = Self::default();
        res.extend(iter);
        res
    }
}

impl<'a> IntoIterator for &'a QStringList {
    type Item = &'a crate::core::QString;
    type IntoIter = std::slice::Iter<'a, crate::core::QString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

