use crate::core::{QByteArray, QString};
use std::convert::TryFrom;

cpp! {{
    #include <QVariant>
}}

cpp_class!(
    #[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub unsafe struct QVariant as "QVariant"
);

impl QVariant {
    pub fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "const QVariant*"] -> bool as "bool" { return self->isValid(); })
    }

    pub fn is_null(&self) -> bool {
        cpp!(unsafe [self as "const QVariant*"] -> bool as "bool" { return self->isNull(); })
    }
}

impl From<i32> for QVariant {
    fn from(value: i32) -> Self {
        cpp!(unsafe [value as "int"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<u32> for QVariant {
    fn from(value: u32) -> Self {
        cpp!(unsafe [value as "uint"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<i64> for QVariant {
    fn from(value: i64) -> Self {
        cpp!(unsafe [value as "qint64"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<u64> for QVariant {
    fn from(value: u64) -> Self {
        cpp!(unsafe [value as "quint64"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<bool> for QVariant {
    fn from(value: bool) -> Self {
        cpp!(unsafe [value as "bool"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<f32> for QVariant {
    fn from(value: f32) -> Self {
        cpp!(unsafe [value as "float"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<f64> for QVariant {
    fn from(value: f64) -> Self {
        cpp!(unsafe [value as "double"] -> QVariant as "QVariant" { return QVariant(value); })
    }
}

impl From<&QByteArray> for QVariant {
    fn from(value: &QByteArray) -> Self {
        cpp!(unsafe [value as "const QByteArray*"] -> QVariant as "QVariant" {
            return QVariant(value);
        })
    }
}

impl<'a> From<&'a str> for QVariant {
    fn from(value: &'a str) -> Self {
        // inlined to reduce FFI overhead
        let bytes = value.as_bytes();
        let data: *const u8 = bytes.as_ptr();
        let len: usize = bytes.len();
        cpp!(unsafe [data as "const char*", len as "size_t"] -> QVariant as "QVariant" {
            return QVariant(QString::fromUtf8(data, len));
        })
    }
}

impl From<String> for QVariant {
    fn from(value: String) -> Self {
        let value: &str = &value;
        value.into()
    }
}

impl From<&QString> for QVariant {
    fn from(value: &QString) -> Self {
        cpp!(unsafe [value as "const QString*"] -> QVariant as "QVariant" {
            return QVariant(value);
        })
    }
}

impl From<&QVariant> for bool {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> bool as "bool" {
            return value->toBool();
        })
    }
}

impl TryFrom<&QVariant> for i32 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> i32 as "int" {
            return value->toInt(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&QVariant> for u32 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> u32 as "uint" {
            return value->toUInt(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&QVariant> for i64 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> i64 as "qint64" {
            return value->toLongLong(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&QVariant> for u64 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> u64 as "quint64" {
            return value->toULongLong(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&QVariant> for f32 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> f32 as "float" {
            return value->toFloat(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&QVariant> for f64 {
    type Error = ();

    fn try_from(value: &QVariant) -> Result<Self, ()> {
        let mut ok: bool = false;
        let res = cpp!(unsafe [value as "const QVariant*", mut ok as "bool"] -> f64 as "double" {
            return value->toDouble(&ok);
        });
        if ok {
            Ok(res)
        } else {
            Err(())
        }
    }
}

impl From<&QVariant> for QByteArray {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> QByteArray as "QByteArray" {
            return value->toByteArray();
        })
    }
}

impl From<&QVariant> for QString {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> QString as "QString" {
            return value->toString();
        })
    }
}
