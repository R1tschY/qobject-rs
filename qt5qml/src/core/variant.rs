use crate::core::{QByteArray, QString};
use std::convert::TryFrom;
use std::fmt;

cpp! {{
    #include <QVariant>
}}

cpp_class!(
    #[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub unsafe struct QVariant as "QVariant"
);

impl QVariant {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_valid(&self) -> bool {
        cpp!(unsafe [self as "const QVariant*"] -> bool as "bool" { return self->isValid(); })
    }

    pub fn is_null(&self) -> bool {
        cpp!(unsafe [self as "const QVariant*"] -> bool as "bool" { return self->isNull(); })
    }
}

macro_rules! qvariant_from_delegation {
    ($ty:ty, $delegate:ty) => {
        impl From<$ty> for QVariant {
            fn from(value: $ty) -> Self {
                QVariant::from(value as $delegate)
            }
        }

        impl TryFrom<&QVariant> for $ty {
            type Error = ();

            fn try_from(value: &QVariant) -> Result<Self, ()> {
                <$delegate>::try_from(value).map(|r| r as Self)
            }
        }

        impl TryFrom<QVariant> for $ty {
            type Error = ();

            fn try_from(value: QVariant) -> Result<Self, ()> {
                <$delegate>::try_from(value).map(|r| r as Self)
            }
        }
    };
}

qvariant_from_delegation!(i8, i32);
qvariant_from_delegation!(i16, i32);
qvariant_from_delegation!(u8, u32);
qvariant_from_delegation!(u16, u32);

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

macro_rules! qvariant_try_from_value {
    ($ty:ty) => {
        impl TryFrom<QVariant> for $ty {
            type Error = ();

            fn try_from(value: QVariant) -> Result<Self, ()> {
                <$ty>::try_from(&value)
            }
        }
    };
}

macro_rules! qvariant_from_value {
    ($ty:ty) => {
        impl From<QVariant> for $ty {
            fn from(value: QVariant) -> Self {
                <$ty>::from(&value)
            }
        }
    };
}

impl From<&QVariant> for bool {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> bool as "bool" {
            return value->toBool();
        })
    }
}
qvariant_from_value!(bool);

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
qvariant_try_from_value!(i32);

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
qvariant_try_from_value!(u32);

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
qvariant_try_from_value!(i64);

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
qvariant_try_from_value!(u64);

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
qvariant_try_from_value!(f32);

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
qvariant_try_from_value!(f64);

impl From<&QVariant> for QByteArray {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> QByteArray as "QByteArray" {
            return value->toByteArray();
        })
    }
}
qvariant_from_value!(QByteArray);

impl From<&QVariant> for QString {
    fn from(value: &QVariant) -> Self {
        cpp!(unsafe [value as "const QVariant*"] -> QString as "QString" {
            return value->toString();
        })
    }
}
qvariant_from_value!(QString);

impl From<&QVariant> for String {
    fn from(value: &QVariant) -> Self {
        QString::decode(
            cpp!(unsafe [value as "const QVariant*"] -> QByteArray as "QByteArray" {
                return value->toString().toUtf8();
            }),
        )
    }
}
qvariant_from_value!(String);

impl fmt::Debug for QVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String = cpp!(unsafe [self as "const QVariant*"] -> QString as "QString" {
            QString buffer;
            QDebug(&buffer).nospace() << *self;
            return buffer;
        })
        .into();
        f.write_str(&str)
    }
}
