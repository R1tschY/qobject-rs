use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::{Debug, Display};

cpp! {{
    #include <QString>
    #include <QByteArray>
}}

cpp_class!(
    #[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub unsafe struct QString as "QString"
);

impl QString {
    #[inline]
    pub fn new() -> Self {
        QString::default()
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
        let data: *const u8 = bytes.as_ptr();
        let len: usize = bytes.len();
        cpp!(unsafe [data as "const char*", len as "size_t"] -> QString as "QString" {
            return QString::fromUtf8(data, len);
        })
    }

    pub fn from_utf16(bytes: &[u16]) -> Self {
        let data: *const u16 = bytes.as_ptr();
        let len: usize = bytes.len();
        cpp!(unsafe [data as "const ushort*", len as "size_t"] -> QString as "QString" {
            return QString::fromUtf16(data, len);
        })
    }

    pub unsafe fn from_utf16_unchecked(bytes: &[u16]) -> Self {
        let data: *const u16 = bytes.as_ptr();
        let len: usize = bytes.len();
        cpp!(unsafe [data as "const QChar*", len as "size_t"] -> QString as "QString" {
            return QString(data, len);
        })
    }

    pub fn to_utf8(&self) -> QByteArray {
        cpp!(unsafe [self as "const QString*"] -> QByteArray as "QByteArray" {
            return self->toUtf8();
        })
    }

    pub fn to_string(&self) -> String {
        Self::decode(self.to_utf8())
    }

    pub fn utf16(&self) -> &[u16] {
        unsafe {
            let mut len: usize = 0;
            let data = cpp!(
                [self as "const QString*", mut len as "size_t"] -> *const u16 as "const QChar*" {
                    len = self->size();
                    return self->constData();
                }
            );
            std::slice::from_raw_parts(data, len)
        }
    }

    pub fn len(&self) -> usize {
        cpp!(unsafe [self as "const QString*"] -> i32 as "int" {
            return self->size();
        }) as usize
    }

    pub(crate) fn decode(bytes: QByteArray) -> String {
        String::from_utf8(bytes.as_slice().to_vec()).expect("QString with invalid unicode")
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
        (&self.to_string() as &dyn Debug).fmt(f)
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
        QString::from_utf8_option(self.clone())
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

cpp_class!(
    #[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub unsafe struct QByteArray as "QByteArray"
);

impl QByteArray {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let mut len: usize = 0;
            let data = cpp!(
                [self as "const QByteArray*", mut len as "size_t"] -> *const u8 as "const char*" {
                    len = self->size();
                    return self->constData();
                }
            );
            std::slice::from_raw_parts(data, len)
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let data: *const u8 = bytes.as_ptr();
        let len: usize = bytes.len();
        cpp!(unsafe [data as "const char*", len as "size_t"] -> QByteArray as "QByteArray" {
            return QByteArray(data, len);
        })
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
