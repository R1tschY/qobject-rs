use qobject_compiler::moc::MocConfig;
use qobject_compiler::qobject::TypeRefTrait;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, TypeRef};
use qt5qml::core::{QHashIntQByteArray, QModelIndex, QVariant};

fn main() {
    let core = pkg_config::probe_library("Qt5Core").unwrap();
    let qml = pkg_config::probe_library("Qt5Qml").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &core.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }
    for include in &qml.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("TestObject")
        .inherit(TypeRef::qt_core_object("QAbstractListModel"))
        .method(
            &QObjectMethod::new("rowCount")
                .const_()
                .override_()
                .arg::<&QModelIndex>("parent")
                .ret::<i32>(),
        )
        .method(
            &QObjectMethod::new("data")
                .const_()
                .override_()
                .arg::<&QModelIndex>("index")
                .arg::<i32>("role")
                .ret::<QVariant>(),
        )
        .method(
            &QObjectMethod::new("roleNames")
                .const_()
                .override_()
                .ret::<QHashIntQByteArray>(),
        )
        .build(&cpp, &moc);
}
