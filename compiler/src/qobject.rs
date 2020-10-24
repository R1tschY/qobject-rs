use std::ffi::CStr;

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Include {
    System(String),
    Relative(String),
}

#[derive(Clone, Debug)]
pub struct TypeRef {
    cpp: String,
    rust: String,
    include: Option<Include>,
}

impl TypeRef {
    pub fn new(cpp_name: &str, rust_name: &str, include: Option<Include>) -> Self {
        Self {
            cpp: cpp_name.into(),
            rust: rust_name.into(),
            include,
        }
    }

    pub fn void_mut_ptr() -> Self {
        Self {
            cpp: "void*".into(),
            include: None,
            rust: "*mut std::ffi::c_void".into(),
        }
    }

    pub fn qobject() -> Self {
        Self {
            cpp: "QObject".into(),
            rust: "qt5qml::core::QObject".into(),
            include: Some(Include::System("QObject".into())),
        }
    }

    pub fn generated(name: &str) -> Self {
        Self {
            cpp: name.into(),
            rust: name.into(),
            include: None,
        }
    }

    pub fn qobject_ptr() -> Self {
        Self {
            cpp: "QObject*".into(),
            rust: "*mut qt5qml::core::QObject".into(),
            include: Some(Include::System("QObject".into())),
        }
    }

    pub fn qt_core_object(class_name: &str) -> Self {
        Self {
            cpp: class_name.into(),
            rust: format!("qt5qml::core::{}", class_name),
            include: Some(Include::System(class_name.into())),
        }
    }

    pub fn primitive<T: TypeRefTrait>() -> Self {
        T::type_ref()
    }

    pub fn with_mut_ptr(&self) -> Self {
        Self {
            cpp: format!("{}*", self.cpp),
            include: self.include.clone(),
            rust: format!("*mut {}", self.rust),
        }
    }

    pub fn with_const_ptr(&self) -> Self {
        Self {
            cpp: format!("const {}*", self.cpp),
            include: self.include.clone(),
            rust: format!("*const {}", self.rust),
        }
    }

    pub fn with_mut_ref(&self) -> Self {
        Self {
            cpp: format!("{}&", self.cpp),
            include: self.include.clone(),
            rust: format!("&mut {}", self.rust),
        }
    }

    pub fn with_const_ref(&self) -> Self {
        Self {
            cpp: format!("const {}&", self.cpp),
            include: self.include.clone(),
            rust: format!("&{}", self.rust),
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
}

pub trait TypeRefTrait {
    fn type_ref() -> TypeRef;
}

macro_rules! impl_type_ref_trait {
    ($rust:ty => $cpp:expr, $include:expr) => {
        impl TypeRefTrait for $rust {
            fn type_ref() -> TypeRef {
                TypeRef::new(
                    $cpp.into(),
                    stringify!($rust).into(),
                    Some(Include::System($include.into())),
                )
            }
        }
    };
    ($rust:ty => $cpp:expr) => {
        impl TypeRefTrait for $rust {
            fn type_ref() -> TypeRef {
                TypeRef::new($cpp.into(), stringify!($rust).into(), None)
            }
        }
    };
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

impl_type_ref_trait!(i8 => "int8_t", "cstdint");
impl_type_ref_trait!(u8 => "uint8_t", "cstdint");
impl_type_ref_trait!(i16 => "int16_t", "cstdint");
impl_type_ref_trait!(u16 => "uint16_t", "cstdint");
impl_type_ref_trait!(i32 => "int32_t", "cstdint");
impl_type_ref_trait!(u32 => "uint32_t", "cstdint");
impl_type_ref_trait!(i64 => "int64_t", "cstdint");
impl_type_ref_trait!(u64 => "uint64_t", "cstdint");
impl_type_ref_trait!(f32 => "float");
impl_type_ref_trait!(f64 => "double");
impl_type_ref_trait!(std::os::raw::c_void => "void");
impl_type_ref_trait!(qt5qml::core::QModelIndex => "QModelIndex", "QModelIndex");
impl_type_ref_trait!(qt5qml::core::QString => "QString", "QString");
impl_type_ref_trait!(qt5qml::core::QByteArray => "QByteArray", "QByteArray");
impl_type_ref_trait!(qt5qml::core::QVariant => "QVariant", "QVariant");
impl_type_ref_trait!(qt5qml::core::QHashIntQByteArray => "QHash<int, QByteArray>", "QHash");

impl TypeRefTrait for &CStr {
    fn type_ref() -> TypeRef {
        TypeRef::new(
            "const char*".into(),
            "*const std::os::raw::c_char".into(),
            None,
        )
    }
}

impl TypeRefTrait for &mut CStr {
    fn type_ref() -> TypeRef {
        TypeRef::new("char*".into(), "*mut std::os::raw::c_char".into(), None)
    }
}

#[derive(Clone, Debug)]
pub struct QObjectProp {
    pub(crate) name: String,
    pub(crate) type_ref: TypeRef,
    pub(crate) getter: Option<String>,
    pub(crate) setter: Option<String>,
    pub(crate) signal: Option<String>,
    pub(crate) const_: bool,
}

impl QObjectProp {
    pub fn new(type_ref: &TypeRef, name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_ref: type_ref.clone(),
            getter: None,
            setter: None,
            signal: None,
            const_: false,
        }
    }

    pub fn new_const(type_ref: &TypeRef, name: &str, getter: &str) -> Self {
        Self {
            name: name.to_string(),
            type_ref: type_ref.clone(),
            getter: Some(getter.into()),
            setter: None,
            signal: None,
            const_: true,
        }
    }

    pub fn new_readonly(type_ref: &TypeRef, name: &str, getter: &str, signal: &str) -> Self {
        Self {
            name: name.to_string(),
            type_ref: type_ref.clone(),
            getter: Some(getter.into()),
            setter: None,
            signal: Some(signal.into()),
            const_: false,
        }
    }

    pub fn new_readwrite(
        type_ref: &TypeRef,
        name: &str,
        getter: &str,
        setter: &str,
        signal: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            type_ref: type_ref.clone(),
            getter: Some(getter.into()),
            setter: Some(setter.into()),
            signal: Some(signal.into()),
            const_: false,
        }
    }

    pub fn read<T: Into<String>>(&mut self, getter: T) -> &mut Self {
        self.getter = Some(getter.into());
        self
    }

    pub fn write<T: Into<String>>(&mut self, setter: T) -> &mut Self {
        self.setter = Some(setter.into());
        self
    }

    pub fn notify<T: Into<String>>(&mut self, signal: T) -> &mut Self {
        self.signal = Some(signal.into());
        self
    }

    pub fn const_(&mut self) -> &mut Self {
        self.const_ = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct QObjectSignal {
    pub(crate) name: String,
    pub(crate) args: Vec<(String, TypeRef)>,
}

impl QObjectSignal {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            args: vec![],
        }
    }

    pub fn arg(mut self, name: &str, type_ref: &TypeRef) -> Self {
        self.args.push((name.into(), type_ref.clone()));
        self
    }
}

#[derive(Clone, Debug)]
pub struct QObjectMethod {
    pub(crate) name: String,
    pub(crate) ffi_name: Option<String>,
    pub(crate) args: Vec<(String, TypeRef)>,
    pub(crate) rtype: Option<TypeRef>,
    pub(crate) scriptable: bool,
    pub(crate) invokable: bool,
    pub(crate) const_: bool,
    pub(crate) override_: bool,
}

impl QObjectMethod {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ffi_name: None,
            args: vec![],
            rtype: None,
            scriptable: false,
            invokable: false,
            const_: false,
            override_: false,
        }
    }

    pub fn arg<T: TypeRefTrait>(mut self, name: &str) -> Self {
        self.args.push((name.into(), T::type_ref()));
        self
    }

    pub fn arg_with_type(mut self, name: &str, type_ref: &TypeRef) -> Self {
        self.args.push((name.into(), type_ref.clone()));
        self
    }

    pub fn ret<T: TypeRefTrait>(mut self) -> Self {
        self.rtype = Some(T::type_ref());
        self
    }

    pub fn ret_type(mut self, type_ref: &TypeRef) -> Self {
        self.rtype = Some(type_ref.clone());
        self
    }

    pub fn scriptable(mut self) -> Self {
        self.scriptable = true;
        self
    }

    pub fn invokable(mut self) -> Self {
        self.invokable = true;
        self
    }

    pub fn const_(mut self) -> Self {
        self.const_ = true;
        self
    }

    pub fn override_(mut self) -> Self {
        self.override_ = true;
        self
    }

    pub(crate) fn attach(mut self, cls: &QObjectConfig) -> Self {
        self.ffi_name = Some(format!("Qffi_{}_{}", cls.name, self.name));
        self
    }

    pub fn get_ffi_name(&self) -> &str {
        self.ffi_name
            .as_ref()
            .expect("method was not attached to class")
    }
}

#[derive(Clone, Debug)]
pub struct QObjectConfig {
    pub(crate) name: String,
    pub(crate) base_class: TypeRef,
    pub(crate) properties: Vec<QObjectProp>,
    pub(crate) methods: Vec<QObjectMethod>,
    pub(crate) slots: Vec<QObjectMethod>,
    pub(crate) signals: Vec<QObjectSignal>,
    pub(crate) qml: bool,
}

impl QObjectConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            base_class: TypeRef::qobject(),
            properties: vec![],
            methods: vec![],
            signals: vec![],
            slots: vec![],
            qml: true,
        }
    }

    pub fn inherit<T: Into<TypeRef>>(&mut self, type_ref: T) -> &mut Self {
        self.base_class = type_ref.into();
        self
    }

    pub fn property<T: Into<QObjectProp>>(&mut self, prop: T) -> &mut Self {
        self.properties.push(prop.into());
        self
    }

    pub fn method<T: Into<QObjectMethod>>(&mut self, meth: T) -> &mut Self {
        self.methods.push(meth.into().attach(self));
        self
    }

    pub fn signal<T: Into<QObjectSignal>>(&mut self, signal: T) -> &mut Self {
        self.signals.push(signal.into());
        self
    }

    pub fn slot<T: Into<QObjectMethod>>(&mut self, slot: T) -> &mut Self {
        self.slots.push(slot.into());
        self
    }

    /// Generate qmlRegisterType function
    pub fn qml(&mut self, value: bool) -> &mut Self {
        self.qml = value;
        self
    }
}
