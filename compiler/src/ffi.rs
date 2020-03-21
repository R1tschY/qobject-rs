/*
extern "system" {
    pub fn QVariant_fromString(string: *const QString) -> *mut QVariant;
}
*/

use crate::qobject::TypeRef;
use std::borrow::Cow;
use std::collections::HashSet;

pub struct FfiFunction {
    name: String,
    args: Vec<(String, TypeRef)>,
    rtype: Option<TypeRef>,
}

impl FfiFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            args: vec![],
            rtype: None,
        }
    }

    pub fn new_complete(name: &str, args: Vec<(String, TypeRef)>, rtype: Option<TypeRef>) -> Self {
        Self {
            name: name.into(),
            args,
            rtype,
        }
    }

    pub fn arg(&mut self, name: &str, ty: TypeRef) -> &mut Self {
        self.args.push((name.into(), ty));
        self
    }

    pub fn ret(&mut self, ty: TypeRef) -> &mut Self {
        self.rtype = Some(ty);
        self
    }

    pub fn generate_ffi_def(&self) -> String {
        let mut args: Vec<String> = self
            .args
            .iter()
            .map(|arg| format!("{} {}", arg.1.name(), arg.0))
            .collect();
        let rtype: String = if let Some(rty) = &self.rtype {
            if is_builtin_type(rty.name()) {
                rty.name().into()
            } else {
                args.push(format!("{}* out__", rty.name()));
                "void".into()
            }
        } else {
            "void".into()
        };
        format!("extern \"C\" {} {}({});", rtype, self.name, args.join(", "))
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

pub struct FfiBridge {
    pub(crate) functions: Vec<FfiFunction>,
}

impl FfiBridge {
    pub fn new() -> Self {
        Self { functions: vec![] }
    }

    pub fn function(&mut self, func: FfiFunction) -> &mut Self {
        self.functions.push(func);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qobject::{QObjectConfig, QObjectMethod};

    #[test]
    fn test_cpp_def_no_args() {
        let def = &FfiFunction::new("test").generate_ffi_def();
        assert_eq!("extern \"C\" void Qffi_Test_test();", def);
    }

    #[test]
    fn test_cpp_def_one_arg() {
        let def = &FfiFunction::new("test")
            .arg("arg0", TypeRef::qtobject("CppType"))
            .generate_ffi_def();
        assert_eq!("extern \"C\" void Qffi_Test_test(CppType arg0);", def);
    }

    #[test]
    fn test_cpp_def_many_args() {
        let def = &FfiFunction::new("test")
            .arg("arg0", TypeRef::qtobject("CppType0"))
            .arg("arg1", TypeRef::qtobject("CppType1"))
            .generate_ffi_def();
        assert_eq!(
            "extern \"C\" void Qffi_Test_test(CppType0 arg0, CppType1 arg1);",
            def
        );
    }

    #[test]
    fn test_cpp_def_with_return() {
        let def = &FfiFunction::new("test")
            .arg("arg0", TypeRef::qtobject("CppType0"))
            .arg("arg1", TypeRef::qtobject("CppType1"))
            .ret(TypeRef::qtobject("RetCppType"))
            .generate_ffi_def();
        assert_eq!(
            "extern \"C\" void Qffi_Test_test(CppType0 arg0, CppType1 arg1, RetCppType* out__);",
            def
        );
    }
}
