use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::CStr;
use std::fmt::Write;
use std::iter::FromIterator;

use crate::ffi::{FfiBridge, FfiFunction, ImplCode};
use crate::qobject::{QObjectConfig, QObjectMethod, QObjectProp, QObjectSignal};
use crate::typeref::{Include, TypeRef};
use crate::utils::to_snake_case;
use std::fmt;

pub trait Dependent {
    fn dependencies(&self, includes: &mut HashSet<Include>);
}

impl Dependent for QObjectProp {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        if let Some(include) = self.type_ref.include() {
            includes.insert(include.clone());
        }
    }
}

impl Dependent for QObjectMethod {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        if let Some(rtype) = &self.rtype {
            if let Some(include) = rtype.include() {
                includes.insert(include.clone());
            }
        }
        includes.extend(self.args.iter().flat_map(|(_, ty)| ty.include().clone()));
    }
}

impl Dependent for QObjectSignal {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        includes.extend(self.args.iter().flat_map(|(_, ty)| ty.include().clone()));
    }
}

impl Dependent for QObjectConfig {
    fn dependencies(&self, includes: &mut HashSet<Include>) {
        self.properties
            .iter()
            .for_each(|p| p.dependencies(includes));
        self.methods.iter().for_each(|p| p.dependencies(includes));
        self.signals.iter().for_each(|p| p.dependencies(includes));
        if let Some(include) = self.base_class.include() {
            includes.insert(include.clone());
        }
        includes.insert(Include::System("utility".into())); // required for std::forward
        if self.qml {
            includes.insert(Include::System("QtQml".into()));
        }
    }
}

trait GenerateCppCode: Dependent {
    fn fill_ffi_functions(&self, ffi: &mut FfiBridge);
    fn generate_forward_definitions(&self, result: &mut String);
    fn generate_classes(&self, result: &mut String, friend_func: &[&FfiFunction]);
    fn generate_implementations(&self, result: &mut String);
}

fn generate_include(include: &Include) -> String {
    match include {
        Include::System(include) => format!("#include <{}>", include),
        Include::Relative(include) => format!("#include \"{}\"", include),
    }
}

fn generate_base_function_def(
    name: &str,
    args: &Vec<(String, TypeRef)>,
    rtype: &Option<TypeRef>,
) -> String {
    format!(
        "{} {}({})",
        rtype
            .as_ref()
            .map(|rt| rt.cpp_type().clone())
            .unwrap_or("void".into()),
        name,
        args.iter()
            .map(|(name, ty)| format!("{} {}", ty.cpp_type(), name))
            .collect::<Vec<String>>()
            .join(", "),
    )
}

fn generate_prop_def(writer: &mut String, prop: &QObjectProp) {
    let read = prop
        .getter
        .as_ref()
        .map(|getter| format!(" READ {}", getter))
        .unwrap_or(String::new());
    let write = prop
        .setter
        .as_ref()
        .map(|setter| format!(" WRITE {}", setter))
        .unwrap_or(String::new());
    let notify = prop
        .signal
        .as_ref()
        .map(|signal| format!(" NOTIFY {}", signal))
        .unwrap_or(String::new());
    let const_ = if prop.const_ { " CONST" } else { "" };

    writeln!(
        writer,
        "  Q_PROPERTY({} {}{}{}{}{});",
        prop.type_ref.cpp_type(),
        prop.name,
        read,
        write,
        notify,
        const_
    );
}

fn generate_method_impl(meth: &QObjectMethod) -> String {
    let scriptable = if meth.scriptable { "Q_SCRIPTABLE " } else { "" };
    let invokable = if meth.invokable { "Q_INVOKABLE " } else { "" };
    let const_ = if meth.const_ { " const" } else { "" };
    let override_ = if meth.override_ { " override" } else { "" };

    format!(
        "  {}{}{}{}{} {{\n    {}\n  }}",
        scriptable,
        invokable,
        generate_base_function_def(&meth.name, &meth.args, &meth.rtype),
        const_,
        override_,
        generate_ffi_impl(meth),
    )
}

fn generate_ffi_impl(meth: &QObjectMethod) -> String {
    let mut params: Vec<&str> = meth.args.iter().map(|arg| &arg.0 as &str).collect();

    if let Some(proxy_class) = &meth.proxy_class {
        return format!(
            "return {}::{}({});",
            proxy_class,
            &meth.name,
            params.join(", ")
        );
    }

    params.insert(0, "_d".into());
    if let Some(rty) = &meth.rtype {
        if rty.return_safe() {
            format!("return {}({});", meth.get_ffi_name(), params.join(", "))
        } else {
            params.push("&out__".into());
            format!(
                "{} out__;\n    {}({});\n    return out__;",
                rty.cpp_type(),
                meth.get_ffi_name(),
                params.join(", ")
            )
        }
    } else {
        format!("{}({});", meth.get_ffi_name(), params.join(", "))
    }
}

fn generate_signal(signal: &QObjectSignal) -> String {
    format!(
        "{};",
        generate_base_function_def(&signal.name, &signal.args, &None),
    )
}

pub fn generate(moc_name: &str, objects: &[&QObjectConfig]) -> (String, String) {
    let mut ffi = FfiBridge::new();
    for obj in objects {
        obj.fill_ffi_functions(&mut ffi);
    }

    (
        generate_cpp(moc_name, objects, &ffi),
        generate_rust(objects, &ffi),
    )
}

fn generate_cpp(moc_name: &str, objects: &[&QObjectConfig], ffi: &FfiBridge) -> String {
    let mut result: String = String::with_capacity(4 * 1024);

    // header
    result.push_str("// Generated by qobject compiler\n");
    result.push('\n');

    // includes
    let mut includes = HashSet::new();
    for obj in objects {
        obj.dependencies(&mut includes);
    }
    let mut includes = Vec::from_iter(includes.into_iter());
    includes.sort();
    for i in &includes {
        result.push_str(&generate_include(i));
        result.push('\n');
    }

    // forward/extern definitions
    result.push('\n');
    for obj in objects {
        obj.generate_forward_definitions(&mut result);
    }
    for function in ffi.get_rust_functions() {
        let _ = writeln!(result, "{}", function.generate_cpp_def());
        result.push('\n');
    }
    for function in ffi.get_cpp_functions() {
        let _ = writeln!(result, "{}", function.generate_cpp_def());
        result.push('\n');
    }

    // classes
    result.push('\n');
    for obj in objects {
        let friends: Vec<&FfiFunction> = ffi
            .get_cpp_functions()
            .iter()
            .filter(|f| f.get_friend_class().map(|f| f == obj.name).unwrap_or(false))
            .collect();
        obj.generate_classes(&mut result, &friends);
    }

    // impls
    result.push('\n');
    for obj in objects {
        obj.generate_implementations(&mut result);
    }
    for function in ffi.get_cpp_functions() {
        if function.get_friend_class().is_none() {
            let _ = writeln!(result, "{}", function.generate_cpp_impl());
        }
    }

    // moc
    result.push('\n');
    let _ = writeln!(result, "#include \"{}\"", moc_name);

    result
}

fn gen_cpp_meth_call(meth: &QObjectMethod) -> String {
    let params: Vec<&str> = meth.args.iter().map(|a| &a.0 as &str).collect();
    match &meth.rtype {
        Some(ref rty) if !rty.return_safe() => {
            return format!(
                "new(out__) {}(std::move(self_.{}({})));",
                rty.cpp_type(),
                &meth.name,
                params.join(", ")
            )
        }
        _ => format!("return self_.{}({});", &meth.name, params.join(", ")),
    }
}

fn gen_rust_meth_call(cls: &str, meth: &QObjectMethod) -> String {
    let params: Vec<&str> = meth.args.iter().map(|a| &a.0 as &str).collect();
    let ret = match &meth.rtype {
        Some(ref rty) if !rty.return_safe() => "*out__ = ",
        _ => "",
    };
    format!(
        "unsafe {{ {}(*(self_ as *mut {}Private)).{}({}) }}",
        ret,
        cls,
        to_snake_case(&meth.name),
        params.join(", ")
    )
}

impl GenerateCppCode for QObjectConfig {
    fn fill_ffi_functions(&self, ffi: &mut FfiBridge) {
        let class_type = TypeRef::new(self.name.clone(), self.name.clone(), false, None);

        for meth in self.methods.iter().chain(self.slots.iter()) {
            if let Some(_proxy_class) = &meth.proxy_class {
                let mut args = meth.args.clone();
                let cls_ref = if meth.const_ {
                    class_type.clone().with_const_ref()
                } else {
                    class_type.clone().with_mut_ref()
                };
                args.insert(0, ("self_".into(), cls_ref));

                ffi.cpp_function(FfiFunction::new_complete(
                    meth.get_ffi_name(),
                    args,
                    meth.rtype.clone(),
                    ImplCode::Cpp(gen_cpp_meth_call(meth)),
                    None,
                ));
            } else {
                let mut args = meth.args.clone();
                args.insert(0, ("self_".into(), TypeRef::void_mut_ptr()));

                ffi.rust_function(FfiFunction::new_complete(
                    meth.get_ffi_name(),
                    args,
                    meth.rtype.clone(),
                    ImplCode::Rust(gen_rust_meth_call(&self.name, &meth)),
                    None,
                ));
            }
        }

        ffi.rust_function(FfiFunction::new_complete(
            &format!("Qffi_{}_private_new", &self.name),
            vec![(
                "qobject".into(),
                TypeRef::generated(self.name.clone()).with_mut_ptr(),
            )],
            Some(TypeRef::void_mut_ptr()),
            ImplCode::Rust(format!(
                "Box::into_raw(Box::new({}Private::new(qobject))) as *mut std::ffi::c_void",
                &self.name
            )),
            None,
        ));
        ffi.rust_function(FfiFunction::new_complete(
            &format!("Qffi_{}_private_delete", self.name),
            vec![("self_".into(), TypeRef::void_mut_ptr())],
            None,
            ImplCode::Rust(format!(
                "unsafe {{ drop(Box::from_raw(self_ as *mut {}Private)) }};",
                &self.name
            )),
            None,
        ));
        ffi.cpp_function(FfiFunction::new_complete(
            &format!("Qffi_{}_new", &self.name),
            vec![("parent".into(), TypeRef::qobject_ptr())],
            Some(class_type.clone().with_mut_ptr()),
            ImplCode::Cpp(format!("return new {}(parent);", &self.name)),
            None,
        ));
        ffi.cpp_function(FfiFunction::new_complete(
            &format!("Qffi_{}_get_private", &self.name),
            vec![("self_".into(), class_type.clone().with_mut_ptr())],
            Some(TypeRef::void_mut_ptr()),
            ImplCode::Cpp("return self_->_d;".to_string()),
            None,
        ));

        for signal in &self.signals {
            let mut args = signal.args.clone();
            args.insert(
                0,
                (
                    "self_".into(),
                    TypeRef::generated(self.name.clone()).with_mut_ptr(),
                ),
            );

            let params: Vec<&str> = signal.args.iter().map(|a| &a.0 as &str).collect();
            let body = format!("Q_EMIT self_->{}({});", signal.name, params.join(", "));
            ffi.cpp_function(FfiFunction::new_complete(
                &format!("Qffi_{}_{}", self.name, signal.name),
                args,
                None,
                ImplCode::Cpp(body.into()),
                None,
            ));
        }

        if self.qml {
            ffi.cpp_function(
                FfiFunction::new(&format!("Qffi_{}_registerType", self.name))
                    .arg::<&CStr>("uri")
                    .arg::<i32>("version_major")
                    .arg::<i32>("version_minor")
                    .arg::<&CStr>("qml_name")
                    .ret::<i32>()
                    .cpp_impl(&format!(
                        "return qmlRegisterType<{}>(uri, version_major, version_minor, qml_name);",
                        self.name
                    )),
            );
        }
    }

    fn generate_forward_definitions(&self, result: &mut String) {
        let _ = writeln!(result, "class {};", self.name);
    }

    fn generate_classes(&self, result: &mut String, friend_funcs: &[&FfiFunction]) {
        // class
        let _ = writeln!(
            result,
            "class {} : public {} {{",
            &self.name,
            self.base_class.cpp_type()
        );
        result.push_str("  Q_OBJECT\n");

        // properties
        result.push('\n');
        for prop in &self.properties {
            let _ = generate_prop_def(result, prop);
        }

        // ctor and dtor
        result.push('\n');
        result.push_str("public:\n");
        let _ = writeln!(
            result,
            "  {0}(QObject* parent = nullptr) \
  : {1}(parent) \
  {{ _d = Qffi_{0}_private_new(this); }}\n",
            &self.name,
            self.base_class.cpp_type()
        );
        let _ = writeln!(
            result,
            "  ~{0}() {{ Qffi_{0}_private_delete(_d); }}",
            &self.name
        );

        // methods
        result.push('\n');
        for meth in &self.methods {
            result.push_str(&generate_method_impl(meth));
            result.push('\n');
        }

        // signals
        result.push('\n');
        result.push_str("Q_SIGNALS:\n");
        for signal in &self.signals {
            result.push_str(&generate_signal(signal));
            result.push('\n');
        }

        // slots
        result.push('\n');
        result.push_str("public Q_SLOTS:\n");
        for slot in &self.slots {
            result.push_str(&generate_method_impl(slot));
            result.push('\n');
        }

        // private member
        result.push('\n');
        result.push_str("public:\n");
        result.push_str("  void* _d;\n");

        // friends
        result.push('\n');
        for friend in friend_funcs {
            writeln!(result, "{}", friend.generate_friend_cpp_impl());
            result.push('\n');
        }

        // class end
        result.push_str("};\n");
    }

    fn generate_implementations(&self, _result: &mut String) {}
}

fn generate_rust(objects: &[&QObjectConfig], ffi: &FfiBridge) -> String {
    let mut result = String::with_capacity(4 * 1024);

    // C++ extern
    result.push_str("extern \"C\" {\n");
    for function in ffi.get_cpp_functions() {
        result.push_str(&function.generate_rust_def());
        result.push('\n');
    }
    result.push_str("}\n");
    result.push('\n');

    // Rust functions
    for function in ffi.get_rust_functions() {
        result.push_str(&function.generate_rust_impl());
        result.push('\n');
    }

    // Objects
    for obj in objects {
        let _ = writeln!(
            result,
            r#"
#[repr(C)]
pub struct {0} {{
    _private: [u8; 0],
}}

impl qt5qml::core::QObjectRef for {0} {{
    fn as_qobject_mut(&mut self) -> &mut qt5qml::core::QObject {{
        unsafe {{ &mut *(self as *mut _ as *mut qt5qml::core::QObject) }}
    }}

    fn as_qobject(&self) -> &qt5qml::core::QObject {{
        unsafe {{ &*(self as *const _ as *const qt5qml::core::QObject) }}
    }}
}}

impl {0} {{
    pub fn new(parent: *mut qt5qml::core::QObject) -> qt5qml::QBox<{0}> {{
        unsafe {{ qt5qml::QBox::from_raw(Qffi_{0}_new(parent)) }}
    }}

    #[allow(unused)]
    fn get_private<'a>(&'a mut self) -> &'a mut {0}Private {{
        unsafe {{ &mut *(Qffi_{0}_get_private(self) as *mut {0}Private) }}
    }}"#,
            obj.name
        );

        result.push('\n');
        for meth in &obj.methods {
            if meth.proxy_class.is_none() {
                continue;
            }

            let mut args: Vec<String> = meth
                .args
                .iter()
                .map(|arg| format!("{}: {}", arg.0, arg.1.rust_type()))
                .collect();
            if meth.const_ {
                args.insert(0, "&self".into());
            } else {
                args.insert(0, "&mut self".into());
            }
            let mut params: Vec<&str> = meth.args.iter().map(|arg| &arg.0 as &str).collect();
            params.insert(0, "self".into());

            match &meth.rtype {
                Some(ref rty) if !rty.return_safe() => {
                    let _ = writeln!(
                        result,
                        r#"
    pub(crate) fn {1}({2}) -> {4} {{
        let mut out__ = std::mem::MaybeUninit::<{4}>::uninit();
        unsafe {{ {0}({3}, out__.as_mut_ptr()); }}
        unsafe {{ out__.assume_init() }}
    }}
"#,
                        meth.get_ffi_name(),
                        to_snake_case(&meth.name),
                        args.join(", "),
                        params.join(", "),
                        rty.rust_type()
                    );
                }
                _ => {
                    let _ = writeln!(
                        result,
                        r#"
    pub(crate) fn {1}({2}) -> {4} {{
        unsafe {{ {0}({3}) }}
    }}
"#,
                        meth.get_ffi_name(),
                        to_snake_case(&meth.name),
                        args.join(", "),
                        params.join(", "),
                        meth.rtype
                            .as_ref()
                            .map(|rty| rty.rust_type())
                            .unwrap_or("()")
                    );
                }
            };
        }

        result.push('\n');
        for signal in &obj.signals {
            let mut args: Vec<String> = signal
                .args
                .iter()
                .map(|arg| format!("{}: {}", arg.0, arg.1.rust_type()))
                .collect();
            args.insert(0, "&mut self".into());
            let mut params: Vec<&str> = signal.args.iter().map(|arg| &arg.0 as &str).collect();
            params.insert(0, "self".into());
            let _ = writeln!(
                result,
                r#"
    pub(crate) fn {2}({3}) {{
        unsafe {{ Qffi_{0}_{1}({4}); }}
    }}
"#,
                obj.name,
                &signal.name,
                to_snake_case(&signal.name),
                args.join(", "),
                params.join(", ")
            );
        }

        result.push('\n');
        if obj.qml {
            let _ = writeln!(
                result,
                r#"
    pub(crate) fn register_type(uri: &std::ffi::CStr, version_major: i32, version_minor: i32, qml_name: &std::ffi::CStr) -> i32 {{
        unsafe {{ Qffi_{0}_registerType(uri.as_ptr(), version_major, version_minor, qml_name.as_ptr()) }}
    }}
"#,
                obj.name,
            );
        }

        result.push_str("}\n");
    }

    result
}

#[cfg(test)]
mod tests {
    use qt5qml::core::QString;

    use super::*;

    #[test]
    fn generate_simple_class() {
        let mut obj = QObjectConfig::new("Dummy");
        let obj_clone = obj.clone();
        let obj = obj
            .inherit(TypeRef::qt_core_object("QObject"))
            .property(
                QObjectProp::new_with_type(TypeRef::qstring(), "dummy")
                    .read("dummy")
                    .notify("dummyChanged"),
            )
            .method(
                QObjectMethod::new("dummy")
                    .ret::<QString>()
                    .attach(&obj_clone),
            );
        let (code, _) = generate("dummy.moc", &[&obj]);

        println!("{}", code);

        assert!(code.contains("#include <QObject>"));
        assert!(code.contains("#include <QString>"));
        assert!(code.contains("Q_OBJECT"));
        assert!(code.contains("Q_PROPERTY(QString dummy READ dummy NOTIFY dummyChanged);"));
        assert!(code.contains("class Dummy : public QObject"));
        assert!(code.contains("QString dummy()"));
        assert!(code.contains("#include \"dummy.moc\""));
        assert!(code.contains("Qffi_Dummy_dummy(_d, &out__);"));
        assert!(code.contains("void Qffi_Dummy_dummy(void* self_, QString* out__);"));
        assert!(code.contains("void* Qffi_Dummy_private_new(Dummy* qobject);"));
        assert!(code.contains("void Qffi_Dummy_private_delete(void* self_);"));
    }

    #[test]
    fn test_cpp_impl() {
        let def = generate_ffi_impl(
            &(QObjectMethod::new("test")
                .arg_with_type("arg0", TypeRef::qt_core_object("CppType0"))
                .arg_with_type("arg1", TypeRef::qt_core_object("CppType1"))
                .attach(&QObjectConfig::new("Test"))),
        );
        assert_eq!("Qffi_Test_test(_d, arg0, arg1);", def.trim());
    }

    #[test]
    fn test_cpp_impl_with_return() {
        let def = generate_ffi_impl(
            &QObjectMethod::new("test")
                .arg_with_type("arg0", TypeRef::qt_core_object("CppType0"))
                .ret_type(TypeRef::qt_core_object("RetCppType"))
                .attach(&QObjectConfig::new("Test")),
        );
        assert_eq!(
            r#"
    RetCppType out__;
    Qffi_Test_test(_d, arg0, &out__);
    return out__;"#
                .trim(),
            def.trim()
        );
    }

    #[test]
    fn test_cpp_class_with_signal() {
        let mut obj = QObjectConfig::new("Dummy");
        let obj = obj
            .inherit(TypeRef::qobject())
            .signal(QObjectSignal::new("testSignal").arg_with_type("arg0", TypeRef::qobject_ptr()));
        let (code, _) = generate("dummy.moc", &[&obj]);

        println!("{}", code);

        assert!(code.contains("void Qffi_Dummy_testSignal(Dummy* self_, QObject* arg0)"));
        assert!(code.contains("Q_EMIT self_->testSignal(arg0)"));
    }

    #[test]
    fn test_cpp_class_with_slot() {
        let mut obj = QObjectConfig::new("Dummy");
        let obj = obj
            .inherit(TypeRef::qobject())
            .slot(QObjectMethod::new("testSlot").arg::<&QString>("arg0"));
        let (code, _) = generate("dummy.moc", &[&obj]);

        println!("{}", code);

        assert!(code.contains("public Q_SLOTS:"));
        assert!(code.contains("void testSlot(const QString& arg0)"));
    }
}
