use qobject_compiler::moc::MocConfig;
use qobject_compiler::typeref::QString;
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
        .property(QObjectProp::new::<i32>("slotCalls").read("slotCalls"))
        .method(QObjectMethod::new("slotCalls").ret::<i32>())
        .slot(QObjectMethod::new("slot"))
        .slot(QObjectMethod::new("slotWithArgs").arg::<&QString>("strArg"))
        .slot(
            QObjectMethod::new("echoSlot")
                .arg::<&QString>("arg")
                .ret::<QString>(),
        )
        .qml(false)
        .build(&cpp, &moc);
}
