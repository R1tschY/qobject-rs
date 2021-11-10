use std::fmt;

use crate::core::QString;
use crate::ffi;
use crate::ffi::QffiWrapper;

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QUrl(crate::ffi::QUrl);
impl_ffi_trait!(QUrl);

impl QUrl {
    pub fn from_qstring(value: &QString) -> Self {
        unsafe { QUrl::create(|dest| ffi::qffi_QUrl_fromString(value.to_inner(), dest)) }
    }

    fn from_local_file_intern(local_file: &QString) -> Self {
        unsafe { QUrl::create(|dest| ffi::qffi_QUrl_fromLocalFile(local_file.to_inner(), dest)) }
    }

    pub fn from_local_file<T: Into<QString>>(local_file: T) -> Self {
        Self::from_local_file_intern(&local_file.into())
    }
}

impl From<QString> for QUrl {
    fn from(value: QString) -> Self {
        Self::from_qstring(&value)
    }
}

impl<'a> From<&'a str> for QUrl {
    fn from(value: &'a str) -> Self {
        let value: QString = value.into();
        Self::from_qstring(&value)
    }
}

impl From<String> for QUrl {
    fn from(value: String) -> Self {
        let value: QString = value.into();
        Self::from_qstring(&value)
    }
}

impl fmt::Debug for QUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String =
            unsafe { QString::create(|dest| ffi::qffi_QUrl_debug(self.to_inner(), dest)).into() };
        f.write_str(&str)
    }
}
