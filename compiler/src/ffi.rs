use crate::qobject::TypeRef;
use std::collections::HashSet;

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
}

impl FfiFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            rtype: None,
            body: ImplCode::Extern,
        }
    }

    pub(crate) fn new_complete(
        name: &str,
        args: Vec<(String, TypeRef)>,
        rtype: Option<TypeRef>,
        body: ImplCode,
    ) -> Self {
        Self {
            name: name.into(),
            args,
            rtype,
            body,
        }
    }

    pub fn arg(&mut self, name: &str, ty: TypeRef) -> &mut Self {
        self.args.push((name.into(), ty));
        self
    }

    pub fn rust_impl(&mut self, body: &str) -> &mut Self {
        self.body = ImplCode::Rust(body.into());
        self
    }

    pub fn cpp_impl(&mut self, body: &str) -> &mut Self {
        self.body = ImplCode::Cpp(body.into());
        self
    }

    pub fn ret(&mut self, ty: TypeRef) -> &mut Self {
        self.rtype = Some(ty);
        self
    }

    pub fn get_type_refs(&self) -> Vec<TypeRef> {
        let mut result: Vec<TypeRef> = self.args.iter().map(|a| a.1.clone()).collect();
        if let Some(rtype) = &self.rtype {
            result.push(rtype.clone());
        }
        result
    }

    fn generate_cpp_sig(&self) -> String {
        let mut args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{} {}", arg.1.cpp_type(), arg.0))
            .collect();
        let rtype: String = if let Some(rty) = &self.rtype {
            if is_builtin_type(rty.cpp_type()) {
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

    fn generate_rust_sig(&self) -> String {
        let mut args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{}: {}", arg.0, arg.1.rust_type()))
            .collect();
        let rtype: String = if let Some(rty) = &self.rtype {
            if is_builtin_type(rty.cpp_type()) {
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
            "#[no_mangle] pub extern {} {{\n  {}\n}}",
            self.generate_rust_sig(),
            body
        )
    }
}

lazy_static! {
    static ref BUILTIN_TYPES: HashSet<&'static str> = {
        let mut types = HashSet::new();
        types.insert("unsigned");
        types.insert("int");
        types.insert("unsigned int");
        types.insert("long");
        types.insert("unsigned long");
        types.insert("short");
        types.insert("unsigned short");
        types.insert("char");
        types.insert("signed char");
        types.insert("unsigned char");
        types.insert("float");
        types.insert("double");
        types.insert("bool");
        types
    };
}

fn is_builtin_type(ty: &str) -> bool {
    ty.ends_with("*") || BUILTIN_TYPES.contains(ty)
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
            .arg("arg0", TypeRef::qtobject("CppType"))
            .generate_cpp_def();
        assert_eq!("extern \"C\" void test(CppType arg0);", def);
    }

    #[test]
    fn test_cpp_def_many_args() {
        let def = &FfiFunction::new("test")
            .arg("arg0", TypeRef::qtobject("CppType0"))
            .arg("arg1", TypeRef::qtobject("CppType1"))
            .generate_cpp_def();
        assert_eq!(def, "extern \"C\" void test(CppType0 arg0, CppType1 arg1);");
    }

    #[test]
    fn test_cpp_def_with_return() {
        let def = &FfiFunction::new("test")
            .arg("arg0", TypeRef::qtobject("CppType0"))
            .arg("arg1", TypeRef::qtobject("CppType1"))
            .ret(TypeRef::qtobject("RetCppType"))
            .generate_cpp_def();
        assert_eq!(
            "extern \"C\" void test(CppType0 arg0, CppType1 arg1, RetCppType* out__);",
            def
        );
    }
}
