#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
pub enum Include {
    System(String),
    Relative(String),
}

#[derive(Clone, Debug)]
pub struct TypeRef {
    name: String,
    include: Option<Include>,
}

impl TypeRef {
    pub fn new(class_name: &str, include: Option<Include>) -> Self {
        Self {
            name: class_name.into(),
            include,
        }
    }

    pub fn void_ptr() -> Self {
        Self {
            name: "void*".into(),
            include: None,
        }
    }

    pub fn qobject_ptr() -> Self {
        Self {
            name: "QObject*".into(),
            include: Some(Include::System("QObject".into())),
        }
    }

    pub fn qtobject_ptr(class_name: &str) -> Self {
        Self {
            name: format!("{}*", class_name),
            include: Some(Include::System(class_name.into())),
        }
    }

    pub fn qtobject(class_name: &str) -> Self {
        Self {
            name: class_name.into(),
            include: Some(Include::System(class_name.into())),
        }
    }

    pub fn with_mut_ptr(&self) -> Self {
        Self {
            name: format!("{}*", self.name),
            include: self.include.clone(),
        }
    }

    pub fn with_const_ptr(&self) -> Self {
        Self {
            name: format!("const {}*", self.name),
            include: self.include.clone(),
        }
    }

    pub fn with_mut_ref(&self) -> Self {
        Self {
            name: format!("{}&", self.name),
            include: self.include.clone(),
        }
    }

    pub fn with_const_ref(&self) -> Self {
        Self {
            name: format!("const {}&", self.name),
            include: self.include.clone(),
        }
    }

    pub fn qstring() -> Self {
        Self::qtobject("QString")
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn include(&self) -> &Option<Include> {
        &self.include
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

    pub fn read<T: Into<String>>(mut self, getter: T) -> Self {
        self.getter = Some(getter.into());
        self
    }

    pub fn write<T: Into<String>>(mut self, setter: T) -> Self {
        self.setter = Some(setter.into());
        self
    }

    pub fn notify<T: Into<String>>(mut self, signal: T) -> Self {
        self.signal = Some(signal.into());
        self
    }

    pub fn const_(mut self) -> Self {
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
    pub(crate) ffi_name: String,
    pub(crate) args: Vec<(String, TypeRef)>,
    pub(crate) rtype: Option<TypeRef>,
    pub(crate) scriptable: bool,
    pub(crate) const_: bool,
    pub(crate) override_: bool,
}

impl QObjectMethod {
    pub fn new(cls: &QObjectConfig, name: &str) -> Self {
        Self {
            name: name.to_string(),
            ffi_name: format!("Qffi_{}_{}", cls.name, name),
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
            base_class: TypeRef::qobject_ptr(),
            properties: vec![],
            methods: vec![],
            signals: vec![],
        }
    }

    pub fn inherit(mut self, type_ref: TypeRef) -> Self {
        self.base_class = type_ref;
        self
    }

    pub fn property<T: Into<QObjectProp>>(mut self, prop: T) -> Self {
        self.properties.push(prop.into());
        self
    }

    pub fn method<T: Into<QObjectMethod>>(mut self, meth: T) -> Self {
        self.methods.push(meth.into());
        self
    }

    pub fn signal<T: Into<QObjectSignal>>(mut self, signal: T) -> Self {
        self.signals.push(signal.into());
        self
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
