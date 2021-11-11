use qobject_compiler::moc::MocConfig;
use qobject_compiler::typeref::{QString, TypeRef, TypeRefTrait};
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp};

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
            QObjectProp::new_with_type(i32::type_ref(), "prop_rw")
                .read("prop_rw")
                .write("set_prop_rw"),
        )
        .property(QObjectProp::new_with_type(TypeRef::qstring(), "prop_r").read("prop_r"))
        .method(QObjectMethod::new("prop_r").ret::<QString>())
        .method(QObjectMethod::new("prop_rw").ret::<i32>())
        .method(QObjectMethod::new("set_prop_rw").arg::<i32>("value"))
        .qml(false)
        .build(&cpp, &moc);
}
