use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectMethod, QObjectProp, QObjectSignal, TypeRef};

fn main() {
    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &config.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("TestObject")
        .signal(&QObjectSignal::new("signal0"))
        .qml(false)
        .build(&cpp, &moc);
}
