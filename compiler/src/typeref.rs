use std::borrow::Cow;
use std::ffi::CStr;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Include {
    System(String),
    Relative(String),
}

#[derive(Clone, Debug)]
pub struct TypeRef {
    cpp: Cow<'static, str>,
    rust: Cow<'static, str>,
    return_safe: bool,
    include: Option<Include>,
}

impl TypeRef {
    pub fn new(
        cpp_name: impl Into<Cow<'static, str>>,
        rust_name: impl Into<Cow<'static, str>>,
        return_safe: bool,
        include: Option<Include>,
    ) -> Self {
        Self {
            cpp: cpp_name.into(),
            rust: rust_name.into(),
            return_safe,
            include,
        }
    }

    pub fn void_mut_ptr() -> Self {
        Self {
            cpp: "void*".into(),
            include: None,
            rust: "*mut std::ffi::c_void".into(),
            return_safe: true,
        }
    }

    pub fn qobject() -> Self {
        Self {
            cpp: "QObject".into(),
            rust: "qt5qml::core::QObject".into(),
            include: Some(Include::System("QObject".into())),
            return_safe: false,
        }
    }

    pub fn generated(name: impl Into<Cow<'static, str>>) -> Self {
        let name = name.into();
        Self {
            cpp: name.clone(),
            rust: name,
            include: None,
            return_safe: false,
        }
    }

    pub fn qobject_ptr() -> Self {
        Self {
            cpp: "QObject*".into(),
            rust: "*mut qt5qml::core::QObject".into(),
            include: Some(Include::System("QObject".into())),
            return_safe: true,
        }
    }

    pub fn qt_core_object(class_name: impl Into<Cow<'static, str>>) -> Self {
        let class_name = class_name.into();
        Self {
            rust: format!("qt5qml::core::{}", &class_name).into(),
            include: Some(Include::System(class_name.to_string())),
            cpp: class_name,
            return_safe: false,
        }
    }

    pub fn from_type<T: TypeRefTrait>() -> Self {
        T::type_ref()
    }

    pub fn with_mut_ptr(self) -> Self {
        Self {
            cpp: format!("{}*", self.cpp).into(),
            include: self.include,
            rust: format!("*mut {}", self.rust).into(),
            return_safe: true,
        }
    }

    pub fn with_const_ptr(self) -> Self {
        Self {
            cpp: format!("const {}*", self.cpp).into(),
            include: self.include,
            rust: format!("*const {}", self.rust).into(),
            return_safe: true,
        }
    }

    pub fn with_mut_ref(self) -> Self {
        Self {
            cpp: format!("{}&", self.cpp).into(),
            include: self.include,
            rust: format!("&mut {}", self.rust).into(),
            return_safe: true,
        }
    }

    pub fn with_const_ref(self) -> Self {
        Self {
            cpp: format!("const {}&", self.cpp).into(),
            include: self.include,
            rust: format!("&{}", self.rust).into(),
            return_safe: true,
        }
    }

    pub fn qstring() -> Self {
        Self::qt_core_object("QString")
    }

    pub fn cpp_type(&self) -> &str {
        &self.cpp
    }

    pub fn rust_type(&self) -> &str {
        &self.rust
    }

    pub fn include(&self) -> &Option<Include> {
        &self.include
    }

    pub fn return_safe(&self) -> bool {
        self.return_safe
    }
}

pub trait TypeRefTrait {
    fn type_ref() -> TypeRef;
}

impl<T: TypeRefTrait> TypeRefTrait for &T {
    fn type_ref() -> TypeRef {
        T::type_ref().with_const_ref()
    }
}

impl<T: TypeRefTrait> TypeRefTrait for &mut T {
    fn type_ref() -> TypeRef {
        T::type_ref().with_mut_ref()
    }
}

impl<T: TypeRefTrait> TypeRefTrait for *const T {
    fn type_ref() -> TypeRef {
        T::type_ref().with_const_ptr()
    }
}

impl<T: TypeRefTrait> TypeRefTrait for *mut T {
    fn type_ref() -> TypeRef {
        T::type_ref().with_mut_ptr()
    }
}

impl TypeRefTrait for &CStr {
    fn type_ref() -> TypeRef {
        TypeRef::new("const char*", "*const std::os::raw::c_char", true, None)
    }
}

impl TypeRefTrait for &mut CStr {
    fn type_ref() -> TypeRef {
        TypeRef::new("char*", "*mut std::os::raw::c_char", true, None)
    }
}

macro_rules! impl_type_ref_trait {
    ($rust:ty => $cpp:expr, $return_safe:expr, $include:expr) => {
        impl TypeRefTrait for $rust {
            fn type_ref() -> TypeRef {
                TypeRef::new(
                    $cpp,
                    stringify!($rust),
                    $return_safe,
                    Some(Include::System($include.into())),
                )
            }
        }
    };
    ($rust:ty : $placeholder:ty => $cpp:expr, $return_safe:expr, $include:expr) => {
        impl TypeRefTrait for $placeholder {
            fn type_ref() -> TypeRef {
                TypeRef::new(
                    $cpp,
                    stringify!($rust),
                    $return_safe,
                    Some(Include::System($include.into())),
                )
            }
        }
    };
    ($rust:ty => $cpp:expr, $return_safe:expr) => {
        impl TypeRefTrait for $rust {
            fn type_ref() -> TypeRef {
                TypeRef::new($cpp, stringify!($rust), $return_safe, None)
            }
        }
    };
}

pub struct QModelIndex;
pub struct QObject;
pub struct QString;
pub struct QByteArray;
pub struct QVariant;
pub struct QHashIntQByteArray;

impl_type_ref_trait!(i8 => "qint8", true, "QtGlobal");
impl_type_ref_trait!(u8 => "quint8", true, "QtGlobal");
impl_type_ref_trait!(i16 => "qint16", true, "QtGlobal");
impl_type_ref_trait!(u16 => "quint16", true, "QtGlobal");
impl_type_ref_trait!(i32 => "qint32", true, "QtGlobal");
impl_type_ref_trait!(u32 => "quint32", true, "QtGlobal");
impl_type_ref_trait!(i64 => "qint64", true, "QtGlobal");
impl_type_ref_trait!(u64 => "quint64", true, "QtGlobal");
impl_type_ref_trait!(f32 => "float", true);
impl_type_ref_trait!(f64 => "double", true);
impl_type_ref_trait!(bool => "bool", true);
impl_type_ref_trait!(std::os::raw::c_void => "void", false);
impl_type_ref_trait!(qt5qml::core::QModelIndex : QModelIndex => "QModelIndex", false, "QModelIndex");
impl_type_ref_trait!(qt5qml::core::QObject : QObject => "QObject", false, "QObject");
impl_type_ref_trait!(qt5qml::core::QString : QString => "QString", false, "QString");
impl_type_ref_trait!(qt5qml::core::QByteArray : QByteArray => "QByteArray", false, "QByteArray");
impl_type_ref_trait!(qt5qml::core::QVariant : QVariant => "QVariant", false, "QVariant");
impl_type_ref_trait!(qt5qml::core::QHashIntQByteArray : QHashIntQByteArray => "QHash<int, QByteArray>", false, "QHash");
