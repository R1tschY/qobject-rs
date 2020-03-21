use qobject_compiler::build::build;
use qobject_compiler::moc::MocConfig;
use qobject_compiler::qobject::QObjectConfig;
use qobject_compiler::CcBuild;
use std::fs;
use std::path::PathBuf;

#[test]
fn main() {
    let temp_file = tempfile::tempdir().unwrap();
    let dir = temp_file.path();
    let path: PathBuf = "input.cpp".into();

    let config = pkg_config::probe_library("Qt5Core").unwrap();

    let mut moc = MocConfig::new();
    let mut cpp = CcBuild::new();
    cpp.cpp(true);
    cpp.out_dir(&dir);
    cpp.host("x86_64-linux-gnu");
    cpp.target("x86_64-linux-gnu");
    cpp.opt_level(0);
    for include in &config.include_paths {
        cpp.include(include);
        moc.include_path(include);
    }

    let obj = QObjectConfig::new("MyQObject");
    build(&cpp, &moc, &path, &obj).unwrap();

    assert!(path.exists());
    assert!(path.with_extension("moc").exists());
    assert!(path.with_file_name("libinput.a").exists());
    println!("{}", fs::read_to_string(&path).unwrap());
}
