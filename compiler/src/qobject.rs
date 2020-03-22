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
    use_: Option<String>,
}

impl TypeRef {
    pub fn new(
        cpp_name: &str,
        rust_name: &str,
        include: Option<Include>,
        use_: Option<String>,
    ) -> Self {
        Self {
            cpp: cpp_name.into(),
            rust: rust_name.into(),
            include,
            use_,
        }
    }

    pub fn void_mut_ptr() -> Self {
        Self {
            cpp: "void*".into(),
            include: None,
            rust: "*mut c_void".into(),
            use_: Some("std::os::raw::c_void".into()),
        }
    }

    pub fn qobject() -> Self {
        Self {
            cpp: "QObject".into(),
            rust: "QObject".into(),
            include: Some(Include::System("QObject".into())),
            use_: Some("qt5qml::sys::QObject".into()),
        }
    }

    pub fn qobject_ptr() -> Self {
        Self {
            cpp: "QObject*".into(),
            rust: "*mut QObject".into(),
            include: Some(Include::System("QObject".into())),
            use_: Some("qt5qml::sys::QObject".into()),
        }
    }

    pub fn qtobject(class_name: &str) -> Self {
        Self {
            cpp: class_name.into(),
            rust: class_name.into(),
            include: Some(Include::System(class_name.into())),
            use_: Some(format!("qt5qml::sys::{}", class_name)),
        }
    }

    pub fn with_mut_ptr(&self) -> Self {
        Self {
            cpp: format!("{}*", self.cpp),
            include: self.include.clone(),
            rust: format!("*mut {}", self.rust),
            use_: self.use_.clone(),
        }
    }

    pub fn with_const_ptr(&self) -> Self {
        Self {
            cpp: format!("const {}*", self.cpp),
            include: self.include.clone(),
            rust: format!("*const {}", self.rust),
            use_: self.use_.clone(),
        }
    }

    pub fn with_mut_ref(&self) -> Self {
        Self {
            cpp: format!("{}&", self.cpp),
            include: self.include.clone(),
            rust: format!("&mut {}", self.rust),
            use_: self.use_.clone(),
        }
    }

    pub fn with_const_ref(&self) -> Self {
        Self {
            cpp: format!("const {}&", self.cpp),
            include: self.include.clone(),
            rust: format!("&{}", self.rust),
            use_: self.use_.clone(),
        }
    }

    pub fn qstring() -> Self {
        Self::qtobject("QString")
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

    pub fn use_(&self) -> Option<&str> {
        self.use_.as_ref().map(|x| x as &str)
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

    pub fn arg(&mut self, name: &str, type_ref: &TypeRef) -> &mut Self {
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
            const_: false,
            override_: false,
        }
    }

    pub fn arg(mut self, name: &str, type_ref: &TypeRef) -> Self {
        self.args.push((name.into(), type_ref.clone()));
        self
    }

    pub fn ret(mut self, type_ref: &TypeRef) -> Self {
        self.rtype = Some(type_ref.clone());
        self
    }

    pub fn scriptable(mut self) -> Self {
        self.scriptable = true;
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
    pub(crate) signals: Vec<QObjectSignal>,
}

impl QObjectConfig {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            base_class: TypeRef::qobject(),
            properties: vec![],
            methods: vec![],
            signals: vec![],
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
}
