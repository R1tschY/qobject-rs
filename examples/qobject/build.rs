use qobject_compiler::moc::MocConfig;
use qobject_compiler::{CcBuild, QObjectBuild, QObjectProp};
use qt5qml::core::QString;

fn main() {
    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    for include in &config.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    QObjectBuild::new("MyQObject")
        .property(QObjectProp::new::<QString>("my_name"))
        .qml(false)
        .build(&cpp, &moc);
}
