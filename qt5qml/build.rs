fn main() {
    let qt5core = pkg_config::probe_library("Qt5Core").unwrap();
    let qt5gui = pkg_config::probe_library("Qt5Gui").unwrap();
    let qt5qml = pkg_config::probe_library("Qt5Qml").unwrap();

    cc::Build::new()
        .cpp(true)
        .includes(qt5core.include_paths)
        .includes(qt5gui.include_paths)
        .includes(qt5qml.include_paths)
        .file("src/ffi/qffi.cpp")
        .compile("qffi");
}
