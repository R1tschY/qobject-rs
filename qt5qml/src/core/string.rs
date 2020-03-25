use std::fmt;
use std::fmt::{Debug, Display, Error, Formatter};

cpp! {{
    #include <QString>
    #include <QByteArray>
    #include <algorithm>
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
}
