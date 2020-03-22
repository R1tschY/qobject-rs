use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, TypeRef};

fn main() {
    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &config.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("TestObject")
        .property(
            &QObjectProp::new(&TypeRef::qstring(), "prop_rw")
                .read("prop_rw")
                .write("set_prop_rw"),
        )
        .property(&QObjectProp::new(&TypeRef::qstring(), "prop_r").read("prop_r"))
        .property(&QObjectProp::new(&TypeRef::qstring(), "prop_w").write("set_prop_w"))
        .method(&QObjectMethod::new("prop_r").ret(&TypeRef::qstring()))
        .method(&QObjectMethod::new("prop_rw").ret(&TypeRef::qstring()))
        .method(
            &QObjectMethod::new("set_prop_rw").arg("value", &TypeRef::qstring().with_const_ref()),
        )
        .method(
            &QObjectMethod::new("set_prop_w").arg("value", &TypeRef::qstring().with_const_ref()),
        )
        .build(&cpp, &moc);
}
