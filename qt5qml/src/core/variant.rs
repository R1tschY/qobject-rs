use crate::core::{QByteArray, QString};
use crate::ffi::*;
use std::convert::TryFrom;
use std::fmt;
use std::mem::transmute;
use std::str::from_utf8_unchecked;

#[repr(C)]
#[derive(Default, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct QVariant(pub(crate) crate::ffi::QVariant);
impl_ffi_trait!(QVariant);

impl QVariant {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { qffi_QVariant_isValid(self.to_inner()) }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        unsafe { qffi_QVariant_isNull(self.to_inner()) }
    }
}

macro_rules! qvariant_from_delegation {
    ($ty:ty, $delegate:ty) => {
        impl From<$ty> for QVariant {
            #[inline]
            fn from(value: $ty) -> Self {
                QVariant::from(value as $delegate)
            }
        }

        impl TryFrom<&QVariant> for $ty {
            type Error = ();

            #[inline]
            fn try_from(value: &QVariant) -> Result<Self, ()> {
                Ok(<$delegate>::try_from(value)? as Self)
            }
        }

        impl TryFrom<QVariant> for $ty {
            type Error = ();

            #[inline]
            fn try_from(value: QVariant) -> Result<Self, ()> {
                Ok(<$delegate>::try_from(value)? as Self)
            }
        }
    };
}

qvariant_from_delegation!(i8, i32);
qvariant_from_delegation!(i16, i32);
qvariant_from_delegation!(u8, u32);
qvariant_from_delegation!(u16, u32);

macro_rules! qvariant_constructors {
    ( $( $fn_name:ident($ty:ty) );* ; ) => {
        $(
            impl From<$ty> for QVariant {
                #[inline]
                fn from(value: $ty) -> Self {
                    unsafe { QVariant::create(|v| $fn_name(value, v)) }
                }
            }
        )*
    };
}

qvariant_constructors! {
    qffi_QVariant_from_int(i32);
    qffi_QVariant_from_uint(u32);
    qffi_QVariant_from_int64(i64);
    qffi_QVariant_from_uint64(u64);
    qffi_QVariant_from_bool(bool);
    qffi_QVariant_from_float(f32);
    qffi_QVariant_from_double(f64);
}

impl From<&QString> for QVariant {
    #[inline]
    fn from(value: &QString) -> Self {
        unsafe { QVariant::create(|v| qffi_QVariant_fromString(value.to_inner(), v)) }
    }
}

impl From<QString> for QVariant {
    #[inline]
    fn from(value: QString) -> Self {
        unsafe { QVariant::create(|v| qffi_QVariant_fromString(value.to_inner(), v)) }
    }
}

impl From<&QByteArray> for QVariant {
    #[inline]
    fn from(value: &QByteArray) -> Self {
        unsafe { QVariant::create(|v| qffi_QVariant_fromByteArray(value.to_inner(), v)) }
    }
}

impl From<QByteArray> for QVariant {
    #[inline]
    fn from(value: QByteArray) -> Self {
        unsafe { QVariant::create(|v| qffi_QVariant_fromByteArray(value.to_inner(), v)) }
    }
}

impl<'a> From<&'a str> for QVariant {
    #[inline]
    fn from(value: &'a str) -> Self {
        // inlined to reduce FFI overhead
        let bytes = value.as_bytes();
        unsafe {
            QVariant::create(|v| {
                qffi_QVariant_fromUtf8(transmute(bytes.as_ptr()), bytes.len() as i32, v)
            })
        }
    }
}

impl From<String> for QVariant {
    #[inline]
    fn from(value: String) -> Self {
        (&value as &str).into()
    }
}

impl<T> From<Option<T>> for QVariant
where
    QVariant: From<T>,
{
    #[inline]
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => QVariant::default(),
        }
    }
}

macro_rules! qvariant_try_converters {
    ( $( $fn_name:ident($ty:ty) );* ; ) => {
        $(
            impl TryFrom<&QVariant> for $ty {
                type Error = ();

                fn try_from(value: &QVariant) -> Result<Self, ()> {
                    let mut ok: bool = false;
                    let res = unsafe { $fn_name(value.to_inner(), &mut ok) };
                    if ok {
                        Ok(res)
                    } else {
                        Err(())
                    }
                }
            }

            impl TryFrom<QVariant> for $ty {
                type Error = ();

                #[inline]
                fn try_from(value: QVariant) -> Result<Self, ()> {
                    <$ty>::try_from(&value)
                }
            }
        )*
    };
}

qvariant_try_converters! {
    qffi_QVariant_toInt(i32);
    qffi_QVariant_toUInt(u32);
    qffi_QVariant_toLongLong(i64);
    qffi_QVariant_toULongLong(u64);
    qffi_QVariant_toFloat(f32);
    qffi_QVariant_toDouble(f64);
}

macro_rules! qvariant_from_value {
    ($ty:ty) => {
        impl From<QVariant> for $ty {
            #[inline]
            fn from(value: QVariant) -> Self {
                <$ty>::from(&value)
            }
        }
    };
}

impl From<&QVariant> for bool {
    #[inline]
    fn from(value: &QVariant) -> Self {
        unsafe { qffi_QVariant_toBool(value.to_inner()) }
    }
}
qvariant_from_value!(bool);

impl From<&QVariant> for QByteArray {
    #[inline]
    fn from(value: &QVariant) -> Self {
        unsafe { QByteArray::create(|v| qffi_QVariant_toByteArray(value.to_inner(), v)) }
    }
}
qvariant_from_value!(QByteArray);

impl From<&QVariant> for QString {
    #[inline]
    fn from(value: &QVariant) -> Self {
        unsafe { QString::create(|v| qffi_QVariant_toString(value.to_inner(), v)) }
    }
}
qvariant_from_value!(QString);

impl From<&QVariant> for String {
    #[inline]
    fn from(value: &QVariant) -> Self {
        unsafe {
            QString::decode(QByteArray::create(|v| {
                qffi_QVariant_toUtf8(value.to_inner(), v)
            }))
        }
    }
}
qvariant_from_value!(String);

impl fmt::Debug for QVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            let string = QByteArray::create(|v| qffi_QVariant_debug(self.to_inner(), v));
            f.write_str(from_utf8_unchecked(string.as_slice()))
        }
    }
}
