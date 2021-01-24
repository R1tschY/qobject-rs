use std::fmt::Write;

use crate::ffi::FfiBridge;
use crate::qobject::QObjectConfig;
use crate::utils::to_snake_case;

pub(crate) fn generate_rust(objects: &[&QObjectConfig], ffi: &FfiBridge) -> String {
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
    #[allow(unused)]
    pub fn new() -> qt5qml::QBox<{0}> {{
        unsafe {{ qt5qml::QBox::from_raw(Qffi_{0}_new(std::ptr::null_mut())) }}
    }}

    #[allow(unused)]
    pub fn new_with_parent(parent: &mut qt5qml::core::QObject) -> *mut {0} {{
        unsafe {{ Qffi_{0}_new(parent) }}
    }}

    #[allow(unused)]
    fn get_private(&mut self) -> &mut {0}Private {{
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
            params.insert(0, "self");

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
            params.insert(0, "self");
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
