use crate::typeref::{TypeRef, TypeRefTrait};

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
    pub fn new<T: TypeRefTrait>(name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_ref: T::type_ref(),
            getter: None,
            setter: None,
            signal: None,
            const_: false,
        }
    }

    pub fn new_with_type(type_ref: TypeRef, name: &str) -> Self {
        Self {
            name: name.to_string(),
            type_ref,
            getter: None,
            setter: None,
            signal: None,
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

    pub fn arg<T: TypeRefTrait>(mut self, name: &str) -> Self {
        self.args.push((name.into(), T::type_ref()));
        self
    }

    pub fn arg_with_type(mut self, name: &str, type_ref: TypeRef) -> Self {
        self.args.push((name.into(), type_ref));
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
    pub(crate) proxy_class: Option<String>,
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
            proxy_class: None,
        }
    }

    pub fn arg<T: TypeRefTrait>(mut self, name: &str) -> Self {
        self.args.push((name.into(), T::type_ref()));
        self
    }

    pub fn arg_with_type(mut self, name: &str, type_ref: TypeRef) -> Self {
        self.args.push((name.into(), type_ref));
        self
    }

    pub fn ret<T: TypeRefTrait>(mut self) -> Self {
        self.rtype = Some(T::type_ref());
        self
    }

    pub fn ret_type(mut self, type_ref: TypeRef) -> Self {
        self.rtype = Some(type_ref);
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

    /// Create proxy that forward call from Rust to C++
    pub fn proxy(mut self, cls: &str) -> Self {
        self.proxy_class = Some(cls.to_string());
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

    pub fn inherit(&mut self, type_ref: TypeRef) -> &mut Self {
        self.base_class = type_ref;
        self
    }

    pub fn property(&mut self, prop: QObjectProp) -> &mut Self {
        self.properties.push(prop);
        self
    }

    pub fn method(&mut self, meth: QObjectMethod) -> &mut Self {
        self.methods.push(meth.attach(self));
        self
    }

    pub fn signal(&mut self, signal: QObjectSignal) -> &mut Self {
        self.signals.push(signal);
        self
    }

    pub fn slot(&mut self, slot: QObjectMethod) -> &mut Self {
        self.slots.push(slot.attach(self));
        self
    }

    /// Generate qmlRegisterType function
    pub fn qml(&mut self, value: bool) -> &mut Self {
        self.qml = value;
        self
    }
}
