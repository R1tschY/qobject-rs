use qobject_compiler::moc::MocConfig;
use qobject_compiler::qobject::TypeRefTrait;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, TypeRef};
use qt5qml::core::{QModelIndex, QVariant};

fn main() {
    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &config.include_paths {
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
        // .method(
        //     &QObjectMethod::new("roleNames")
        //         .const_()
        //         .override_()
        //         .arg::<QModelIndex>("index")
        //         .arg::<i32>("role")
        //         .ret::<QHash_int_QByteArray>(),
        // )
        .qml(false)
        .build(&cpp, &moc);
}
