use crate::typeref::{TypeRef, TypeRefTrait};

pub(crate) enum ImplCode {
    Cpp(String),
    Rust(String),
    Extern,
}

pub(crate) struct FfiFunction {
    name: String,
    args: Vec<(String, TypeRef)>,
    rtype: Option<TypeRef>,
    body: ImplCode,
    friend_class: Option<String>,
}

#[allow(dead_code)]
impl FfiFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            rtype: None,
            body: ImplCode::Extern,
            friend_class: None,
        }
    }

    pub(crate) fn new_complete(
        name: &str,
        args: Vec<(String, TypeRef)>,
        rtype: Option<TypeRef>,
        body: ImplCode,
        friend_class: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            args,
            rtype,
            body,
            friend_class,
        }
    }

    pub fn arg<T: TypeRefTrait>(mut self, name: &str) -> Self {
        self.args.push((name.into(), T::type_ref()));
        self
    }

    pub fn arg_with_type(mut self, name: &str, ty: TypeRef) -> Self {
        self.args.push((name.into(), ty));
        self
    }

    pub fn rust_impl(mut self, body: &str) -> Self {
        self.body = ImplCode::Rust(body.into());
        self
    }

    pub fn cpp_impl(mut self, body: &str) -> Self {
        self.body = ImplCode::Cpp(body.into());
        self
    }

    pub fn ret<T: TypeRefTrait>(mut self) -> Self {
        self.rtype = Some(T::type_ref());
        self
    }

    pub fn ret_type(mut self, ty: TypeRef) -> Self {
        self.rtype = Some(ty);
        self
    }

    pub fn friend_class(mut self, cls: &str) -> Self {
        self.friend_class = Some(cls.into());
        self
    }

    pub fn get_friend_class(&self) -> Option<&str> {
        self.friend_class.as_ref().map(|x| x as &str)
    }

    pub fn get_type_refs(&self) -> Vec<TypeRef> {
        let mut result: Vec<TypeRef> = self.args.iter().map(|a| a.1.clone()).collect();
        if let Some(rtype) = &self.rtype {
            result.push(rtype.clone());
        }
        result
    }

    pub fn generate_cpp_sig(&self) -> String {
        let mut args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{} {}", arg.1.cpp_type(), arg.0))
            .collect();
        let rtype: String = if let Some(rty) = &self.rtype {
            if rty.return_safe() {
                rty.cpp_type().into()
            } else {
                args.push(format!("{}* out__", rty.cpp_type()));
                "void".into()
            }
        } else {
            "void".into()
        };
        format!("{} {}({})", rtype, self.name, args.join(", "))
    }

    pub fn generate_cpp_def(&self) -> String {
        format!("extern \"C\" {};", self.generate_cpp_sig())
    }

    pub fn generate_cpp_impl(&self) -> String {
        let body = match &self.body {
            ImplCode::Cpp(body) => body,
            _ => panic!("No C++ body for {}", self.name),
        };
        format!(
            "extern \"C\" {} {{\n  {}\n}}",
            self.generate_cpp_sig(),
            body
        )
    }

    pub fn generate_friend_cpp_impl(&self) -> String {
        let body = match &self.body {
            ImplCode::Cpp(body) => body,
            _ => panic!("No C++ body for {}", self.name),
        };
        format!("friend {} {{\n  {}\n}}", self.generate_cpp_sig(), body)
    }

    fn generate_rust_sig(&self) -> String {
        let mut args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{}: {}", arg.0, arg.1.rust_type()))
            .collect();
        let rtype: String = if let Some(rty) = &self.rtype {
            if rty.return_safe() {
                rty.rust_type().into()
            } else {
                args.push(format!("out__: *mut {}", rty.rust_type()));
                "()".into()
            }
        } else {
            "()".into()
        };
        format!("fn {}({}) -> {}", self.name, args.join(", "), rtype)
    }

    pub fn generate_rust_def(&self) -> String {
        format!("  pub {};", self.generate_rust_sig())
    }

    pub fn generate_rust_impl(&self) -> String {
        let body = match &self.body {
            ImplCode::Rust(body) => body,
            _ => panic!("No Rust body for {}", self.name),
        };
        format!(
            "#[no_mangle] pub extern \"C\" {} {{\n  {}\n}}",
            self.generate_rust_sig(),
            body
        )
    }
}

pub(crate) struct FfiBridge {
    rust_functions: Vec<FfiFunction>,
    cpp_functions: Vec<FfiFunction>,
}

impl FfiBridge {
    pub fn new() -> Self {
        Self {
            rust_functions: vec![],
            cpp_functions: vec![],
        }
    }

    pub fn rust_function(&mut self, func: FfiFunction) -> &mut Self {
        self.rust_functions.push(func);
        self
    }

    pub fn cpp_function(&mut self, func: FfiFunction) -> &mut Self {
        self.cpp_functions.push(func);
        self
    }

    pub fn get_rust_functions(&self) -> &[FfiFunction] {
        &self.rust_functions
    }

    pub fn get_cpp_functions(&self) -> &[FfiFunction] {
        &self.cpp_functions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpp_def_no_args() {
        let def = &FfiFunction::new("test").generate_cpp_def();
        assert_eq!("extern \"C\" void test();", def);
    }

    #[test]
    fn test_cpp_def_one_arg() {
        let def = &FfiFunction::new("test")
            .arg_with_type("arg0", TypeRef::qt_core_object("CppType"))
            .generate_cpp_def();
        assert_eq!("extern \"C\" void test(CppType arg0);", def);
    }

    #[test]
    fn test_cpp_def_many_args() {
        let def = &FfiFunction::new("test")
            .arg_with_type("arg0", TypeRef::qt_core_object("CppType0"))
            .arg_with_type("arg1", TypeRef::qt_core_object("CppType1"))
            .generate_cpp_def();
        assert_eq!(def, "extern \"C\" void test(CppType0 arg0, CppType1 arg1);");
    }

    #[test]
    fn test_cpp_def_with_return() {
        let def = &FfiFunction::new("test")
            .arg_with_type("arg0", TypeRef::qt_core_object("CppType0"))
            .arg_with_type("arg1", TypeRef::qt_core_object("CppType1"))
            .ret_type(TypeRef::qt_core_object("RetCppType"))
            .generate_cpp_def();
        assert_eq!(
            "extern \"C\" void test(CppType0 arg0, CppType1 arg1, RetCppType* out__);",
            def
        );
    }
}
