use crate::ffi::{FfiBridge, FfiFunction, ImplCode};
use crate::qobject::{Include, QObjectConfig, QObjectMethod, QObjectProp, QObjectSignal, TypeRef};
use std::borrow::Cow;
use std::collections::HashSet;
use std::iter::FromIterator;

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
    }
}

trait GenerateCppCode: Dependent {
    fn fill_ffi_functions(&self, ffi: &mut FfiBridge);
    fn generate_forward_definitions(&self, lines: &mut Vec<Cow<str>>);
    fn generate_classes(&self, lines: &mut Vec<Cow<str>>, friend_func: &[&FfiFunction]);
    fn generate_implementations(&self, lines: &mut Vec<Cow<str>>);
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

fn generate_prop_def(prop: &QObjectProp) -> String {
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

    format!(
        "  Q_PROPERTY({} {}{}{}{}{});",
        prop.type_ref.cpp_type(),
        prop.name,
        read,
        write,
        notify,
        const_
    )
}

fn generate_method_impl(meth: &QObjectMethod) -> String {
    let scriptable = if meth.scriptable { "Q_SCRIPTABLE " } else { "" };
    let const_ = if meth.const_ { " const" } else { "" };
    let override_ = if meth.override_ { " override" } else { "" };

    format!(
        "  {}{}{}{} {{\n    {}\n  }}",
        scriptable,
        generate_base_function_def(&meth.name, &meth.args, &meth.rtype),
        const_,
        override_,
        generate_ffi_impl(meth),
    )
}

fn generate_ffi_impl(meth: &QObjectMethod) -> String {
    let mut params: Vec<String> = meth
        .args
        .iter()
        .map(|arg| format!("std::forward<{}>({})", arg.1.cpp_type(), arg.0))
        .collect();
    params.insert(0, "_d".into());
    if let Some(rty) = &meth.rtype {
        params.push("&out__".into());
        format!(
            "{} out__;\n    {}({});\n    return out__;",
            rty.cpp_type(),
            meth.get_ffi_name(),
            params.join(", ")
        )
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
    let mut lines: Vec<Cow<str>> = vec![];

    // header
    lines.push("// Generated by qobject compiler".into());
    lines.push("".into());

    // includes
    let mut includes = HashSet::new();
    for obj in objects {
        obj.dependencies(&mut includes);
    }
    let mut includes = Vec::from_iter(includes.into_iter());
    includes.sort();
    lines.extend(includes.iter().map(|i| generate_include(i).into()));
    lines.push("".into());

    // forward/extern definitions
    for obj in objects {
        obj.generate_forward_definitions(&mut lines);
    }
    for function in ffi.get_rust_functions() {
        lines.push(function.generate_cpp_def().into());
    }
    for function in ffi.get_cpp_functions() {
        lines.push(function.generate_cpp_def().into());
    }

    // classes
    lines.push("".into());
    for obj in objects {
        let friends: Vec<&FfiFunction> = ffi
            .get_cpp_functions()
            .iter()
            .filter(|f| f.get_friend_class().map(|f| f == obj.name).unwrap_or(false))
            .collect();
        obj.generate_classes(&mut lines, &friends);
    }

    // impls
    lines.push("".into());
    for obj in objects {
        obj.generate_implementations(&mut lines);
    }
    for function in ffi.get_cpp_functions() {
        if function.get_friend_class().is_none() {
            lines.push(function.generate_cpp_impl().into());
        }
    }

    // moc
    lines.push("".into());
    lines.push(format!("#include \"{}\"", moc_name).into());

    lines.join("\n")
}

impl GenerateCppCode for QObjectConfig {
    fn fill_ffi_functions(&self, ffi: &mut FfiBridge) {
        for meth in &self.methods {
            let mut args = meth.args.clone();
            args.insert(0, ("self_".into(), TypeRef::void_mut_ptr()));

            let params: Vec<String> = meth.args.iter().map(|a| a.0.clone()).collect();
            let call = format!(
                "unsafe {{ {}(*(self_ as *mut {}Private)).{}({}) }}",
                if meth.rtype.is_some() {
                    "*out__ = "
                } else {
                    ""
                },
                &self.name,
                &meth.name,
                params.join(", ")
            );
            ffi.rust_function(FfiFunction::new_complete(
                meth.get_ffi_name(),
                args,
                meth.rtype.clone(),
                ImplCode::Rust(call),
                None,
            ));
        }
        ffi.rust_function(FfiFunction::new_complete(
            &format!("Qffi_{}_private_new", &self.name),
            vec![(
                "qobject".into(),
                TypeRef::generated(&self.name).with_mut_ptr(),
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
                "unsafe {{ Box::from_raw(self_ as *mut {}Private) }};",
                &self.name
            )),
            None,
        ));
        ffi.cpp_function(FfiFunction::new_complete(
            &format!("Qffi_{}_new", &self.name),
            vec![("parent".into(), TypeRef::qobject_ptr())],
            Some(TypeRef::qobject_ptr()),
            ImplCode::Cpp(format!("return new {}(parent);", &self.name)),
            None,
        ));

        for signal in &self.signals {
            let mut args = signal.args.clone();
            args.insert(
                0,
                (
                    "self_".into(),
                    TypeRef::generated(&self.name).with_mut_ptr(),
                ),
            );

            let params: Vec<String> = signal.args.iter().map(|a| a.0.clone()).collect();
            let body = format!("Q_EMIT self_->{}({});", signal.name, params.join(", "));
            ffi.cpp_function(FfiFunction::new_complete(
                &format!("Qffi_{}_{}", self.name, signal.name),
                args,
                None,
                ImplCode::Cpp(body.into()),
                Some(self.name.clone()),
            ));
        }
    }

    fn generate_forward_definitions(&self, lines: &mut Vec<Cow<str>>) {
        lines.push(format!("class {};", self.name).into());
    }

    fn generate_classes(&self, lines: &mut Vec<Cow<str>>, friend_funcs: &[&FfiFunction]) {
        lines.push(
            format!(
                "class {} : public {} {{",
                &self.name,
                self.base_class.cpp_type()
            )
            .into(),
        );
        lines.push("  Q_OBJECT".into());
        lines.extend(
            self.properties
                .iter()
                .map(|prop| generate_prop_def(prop).into()),
        );
        lines.push("".into());
        lines.push("public:".into());
        lines.push(
            format!(
                "  {0}(QObject* parent = nullptr) : QObject(parent) {{ _d = Qffi_{0}_private_new(this); }}",
                &self.name
            )
            .into(),
        );
        lines.push(format!("  ~{0}() {{ Qffi_{0}_private_delete(_d); }}", &self.name).into());
        lines.push("".into());
        lines.extend(
            self.methods
                .iter()
                .map(|meth| generate_method_impl(meth).into()),
        );
        lines.push("".into());
        lines.push("Q_SIGNALS:".into());
        lines.extend(
            self.signals
                .iter()
                .map(|signal| generate_signal(signal).into()),
        );
        lines.push("".into());
        lines.push("private:".into());
        lines.push("  void* _d;".into());
        lines.push("".into());
        for friend in friend_funcs {
            lines.push(friend.generate_friend_cpp_impl().into());
        }
        lines.push("};".into());
    }

    fn generate_implementations(&self, _lines: &mut Vec<Cow<str>>) {}
}

fn generate_rust(objects: &[&QObjectConfig], ffi: &FfiBridge) -> String {
    let mut lines: Vec<Cow<str>> = vec![];

    // C++ extern
    lines.push("extern \"C\" {".into());
    for function in ffi.get_cpp_functions() {
        lines.push(function.generate_rust_def().into())
    }
    lines.push("}".into());
    lines.push("".into());

    // Rust functions
    for function in ffi.get_rust_functions() {
        lines.push(function.generate_rust_impl().into())
    }

    // Objects
    for obj in objects {
        lines.push(
            format!(
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
        unsafe {{ qt5qml::QBox::from_raw(Qffi_{0}_new(parent) as *mut _ as *mut {0}) }}
    }}"#,
                obj.name
            )
            .into(),
        );

        lines.push("".into());
        for signal in &obj.signals {
            let mut args: Vec<String> = signal
                .args
                .iter()
                .map(|arg| format!("{}: {}", arg.0, arg.1.rust_type()))
                .collect();
            args.insert(0, "&mut self".into());
            let mut params: Vec<String> = signal.args.iter().map(|arg| arg.0.clone()).collect();
            params.insert(0, "self".into());
            lines.push(
                format!(
                    r#"
    pub(crate) unsafe fn {1}({2}) {{
        use qt5qml::core::QObjectRef;
        Qffi_{0}_{1}({3});
    }}
"#,
                    obj.name,
                    signal.name,
                    args.join(", "),
                    params.join(", ")
                )
                .into(),
            );
        }
        lines.push("}".into());
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_simple_class() {
        let mut obj = QObjectConfig::new("Dummy");
        let obj_clone = obj.clone();
        let obj = obj
            .inherit(TypeRef::qt_core_object("QObject"))
            .property(QObjectProp::new_readonly(
                &TypeRef::qstring(),
                "dummy",
                "dummy",
                "dummyChanged",
            ))
            .method(
                QObjectMethod::new("dummy")
                    .ret(&TypeRef::qstring())
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
        assert!(code.contains("void dummyChanged();"));
        assert!(code.contains("#include \"dummy.moc\""));
        assert!(code.contains("Qffi_Dummy_dummy(_d, &out__);"));
        assert!(code.contains("void Qffi_Dummy_dummy(void* self_, QString* out__);"));
        assert!(code.contains("void* Qffi_Dummy_private_new(QObject* qobject);"));
        assert!(code.contains("void Qffi_Dummy_private_delete(void* self_);"));
    }

    #[test]
    fn test_cpp_impl() {
        let def = generate_ffi_impl(
            &(QObjectMethod::new("test")
                .arg("arg0", &TypeRef::qt_core_object("CppType0"))
                .arg("arg1", &TypeRef::qt_core_object("CppType1"))
                .attach(&QObjectConfig::new("Test"))),
        );
        assert_eq!(
            "    Qffi_Test_test(_d, std::forward<CppType0>(arg0), std::forward<CppType1>(arg1));",
            def
        );
    }

    #[test]
    fn test_cpp_impl_with_return() {
        let def = generate_ffi_impl(
            &QObjectMethod::new("test")
                .arg("arg0", &TypeRef::qt_core_object("CppType0"))
                .ret(&TypeRef::qt_core_object("RetCppType"))
                .attach(&QObjectConfig::new("Test")),
        );
        assert_eq!(
            r#"
    RetCppType out__;
    Qffi_Test_test(_d, std::forward<CppType0>(arg0), &out__);
    return out__;"#
                .trim(),
            def.trim()
        );
    }

    #[test]
    fn test_cpp_class_with_signal() {
        let mut obj = QObjectConfig::new("Dummy");
        let obj_clone = obj.clone();
        let obj = obj
            .inherit(TypeRef::qobject())
            .signal(QObjectSignal::new("testSignal").arg("arg0", &TypeRef::qobject_ptr()));
        let (code, _) = generate("dummy.moc", &[&obj]);

        println!("{}", code);

        assert!(code.contains("friend void Qffi_Dummy_testSignal(Dummy* self_, QObject* arg0)"));
        assert!(code.contains("Q_EMIT self_->testSignal(arg0)"));
    }
}
