use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, Include, QObjectBuild, QObjectMethod, TypeRef};
use qt5qml::core::{QObject, QString};

fn main() {
    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &config.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("TestObject")
        .method(QObjectMethod::new("methodSafeReturn").ret::<i32>())
        .method(QObjectMethod::new("methodUnsafeReturn").ret::<QString>())
        .method(
            QObjectMethod::new("methodWithArgs")
                .arg::<&QString>("value1")
                .arg::<u64>("value2")
                .ret::<i32>(),
        )
        .method(QObjectMethod::new("customEvent").override_().arg_with_type(
            "event",
            TypeRef::new(
                "QEvent*",
                "*mut std::ffi::c_void",
                true,
                Some(Include::System("QEvent".to_string())),
            ),
        ))
        .method(
            QObjectMethod::new("sender")
                .const_()
                .proxy("QObject")
                .ret::<*mut QObject>(),
        )
        .method(
            QObjectMethod::new("objectName")
                .const_()
                .proxy("QObject")
                .ret::<QString>(),
        )
        .method(QObjectMethod::new("dumpObjectInfo").proxy("QObject"))
        .qml(false)
        .build(&cpp, &moc);
}
