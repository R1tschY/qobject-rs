use std::borrow::Cow;
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display};
use std::mem::MaybeUninit;
use std::os::raw::c_char;

fn init_ffi_struct<T, F>(f: F) -> T
where
    F: Fn(*mut T) -> (),
{
    unsafe {
        let mut ret = MaybeUninit::uninit();
        f(ret.as_mut_ptr());
        ret.assume_init()
    }
}

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QString(crate::ffi::QString);

impl QString {
    #[inline]
    pub fn new() -> Self {
        Self(crate::ffi::QString::new())
    }

    pub fn from_utf8_option(input: Option<&str>) -> Self {
        if let Some(ref s) = input {
            QString::from_utf8(s)
        } else {
            QString::new()
        }
    }

    pub fn from_utf8(input: &str) -> Self {
        let bytes = input.as_bytes();
        Self(init_ffi_struct(|dest| unsafe {
            crate::ffi::qffi_QString_fromUtf8(
                bytes.as_ptr() as *const c_char,
                bytes.len() as i32,
                dest,
            )
        }))
    }

    pub fn from_utf16(bytes: &[u16]) -> Self {
        Self(init_ffi_struct(|dest| unsafe {
            crate::ffi::qffi_QString_fromUtf16(bytes.as_ptr(), bytes.len() as i32, dest)
        }))
    }

    pub unsafe fn from_utf16_unchecked(bytes: &[u16]) -> Self {
        Self(init_ffi_struct(|dest| unsafe {
            crate::ffi::qffi_QString_fromUtf16Unchecked(bytes.as_ptr(), bytes.len() as i32, dest)
        }))
    }

    #[inline]
    pub fn to_utf8(&self) -> QByteArray {
        QByteArray(init_ffi_struct(|dest| unsafe {
            crate::ffi::qffi_QString_toUtf8(&self.0, dest)
        }))
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&self) -> String {
        Self::decode(self.to_utf8())
    }

    pub fn utf16(&self) -> &[u16] {
        unsafe {
            let mut len: i32 = 0;
            let data = crate::ffi::qffi_QString_utf16(&self.0, &mut len);
            std::slice::from_raw_parts(data, len as usize)
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        unsafe { crate::ffi::qffi_QString_size(&self.0) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_null(&self) -> bool {
        unsafe { crate::ffi::qffi_QString_isNull(&self.0) }
    }

    pub(crate) fn decode(bytes: QByteArray) -> String {
        String::from_utf8(bytes.as_slice().to_vec()).expect("QString with invalid unicode")
    }
}

impl PartialOrd for QString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QString {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { crate::ffi::qffi_QString_compare(&self.0, &other.0) } {
            x if x < 0 => Ordering::Less,
            x if x == 0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl From<String> for QString {
    #[inline]
    fn from(value: String) -> Self {
        QString::from_utf8(&value)
    }
}

impl<'a> From<&'a str> for QString {
    #[inline]
    fn from(value: &'a str) -> Self {
        QString::from_utf8(value)
    }
}

impl<'a> From<&'a String> for QString {
    #[inline]
    fn from(value: &'a String) -> Self {
        QString::from_utf8(value)
    }
}

impl From<QString> for String {
    #[inline]
    fn from(value: QString) -> Self {
        value.to_string()
    }
}

impl From<&QString> for String {
    #[inline]
    fn from(value: &QString) -> Self {
        value.to_string()
    }
}

impl Display for QString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Debug for QString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.to_string(), f)
    }
}

pub trait ToQString {
    fn to_qstring(&self) -> QString;
}

impl ToQString for &str {
    fn to_qstring(&self) -> QString {
        QString::from_utf8(&self)
    }
}

impl ToQString for String {
    fn to_qstring(&self) -> QString {
        QString::from_utf8(&self)
    }
}

impl<'a> ToQString for Cow<'a, str> {
    fn to_qstring(&self) -> QString {
        QString::from_utf8(&self)
    }
}

impl ToQString for Option<&str> {
    fn to_qstring(&self) -> QString {
        QString::from_utf8_option(*self)
    }
}

impl ToQString for Option<String> {
    fn to_qstring(&self) -> QString {
        QString::from_utf8_option(self.as_ref().map(|s| &s as &str))
    }
}

impl<'a> ToQString for Option<Cow<'a, str>> {
    fn to_qstring(&self) -> QString {
        QString::from_utf8_option(self.as_ref().map(|s| &s as &str))
    }
}

//
// QByteArray

#[repr(C)]
#[derive(Clone, Default, Eq, PartialEq)]
pub struct QByteArray(crate::ffi::QByteArray);

impl QByteArray {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let mut len: i32 = 0;
            let data = crate::ffi::qffi_QByteArray_data(&self.0, &mut len) as *const u8;
            std::slice::from_raw_parts(data, len as usize)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(init_ffi_struct(|dest| unsafe {
            crate::ffi::qffi_QByteArray_fromData(
                bytes.as_ptr() as *const i8,
                bytes.len() as i32,
                dest,
            )
        }))
    }
}

impl PartialOrd for QByteArray {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QByteArray {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { crate::ffi::qffi_QByteArray_compare(&self.0, &other.0) } {
            x if x < 0 => Ordering::Less,
            x if x == 0 => Ordering::Equal,
            _ => Ordering::Greater,
        }
    }
}

impl<'a> From<&'a CStr> for QByteArray {
    #[inline]
    fn from(value: &'a CStr) -> Self {
        Self::from_bytes(value.to_bytes())
    }
}

impl<'a> From<&'a str> for QByteArray {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

impl<'a> From<CString> for QByteArray {
    #[inline]
    fn from(value: CString) -> Self {
        Self::from_bytes(value.to_bytes())
    }
}

impl<'a> From<String> for QByteArray {
    #[inline]
    fn from(value: String) -> Self {
        Self::from_bytes(value.as_bytes())
    }
}

impl<'a> From<&'a [u8]> for QByteArray {
    #[inline]
    fn from(value: &'a [u8]) -> Self {
        Self::from_bytes(value)
    }
}
