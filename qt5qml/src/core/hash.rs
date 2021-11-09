// GENERATED -- DO NOT EDIT!!

use crate::core::QByteArray;
use crate::ffi::{qffi_QHashIntQByteArray_insert, qffi_QHashIntQByteArray_size, QffiWrapper};
use std::collections::HashMap;

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QHashIntQByteArray(pub(crate) crate::ffi::QHashIntQByteArray);
impl_ffi_trait!(QHashIntQByteArray);

impl QHashIntQByteArray {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> i32 {
        unsafe { qffi_QHashIntQByteArray_size(&self.0) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, key: &i32, value: &QByteArray) {
        unsafe { qffi_QHashIntQByteArray_insert(&mut self.0, key, value.to_inner()) }
    }
}

impl From<HashMap<i32, QByteArray>> for QHashIntQByteArray {
    fn from(value: HashMap<i32, QByteArray>) -> Self {
        let mut result = Self::new();
        for entry in value {
            result.insert(&entry.0, &entry.1);
        }
        result
    }
}
