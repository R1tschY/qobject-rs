use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, TypeRef};
use qt5qml::core::QString;

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
        .method(&QObjectMethod::new("prop_r").ret::<QString>())
        .method(&QObjectMethod::new("prop_rw").ret::<QString>())
        .method(&QObjectMethod::new("set_prop_rw").arg::<&QString>("value"))
        .method(&QObjectMethod::new("set_prop_w").arg::<&QString>("value"))
        .qml(false)
        .build(&cpp, &moc);
}
