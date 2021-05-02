fn main() {
    let qt5core = pkg_config::probe_library("Qt5Core").unwrap();
    let qt5gui = pkg_config::probe_library("Qt5Gui").unwrap();
    let qt5qml = pkg_config::probe_library("Qt5Qml").unwrap();

    let mut cpp = cpp_build::Config::new();
    for include in &qt5core.include_paths {
        cpp.include(include);
    }
    for include in &qt5gui.include_paths {
        cpp.include(include);
    }
    for include in &qt5qml.include_paths {
        cpp.include(include);
    }
    // Use with RUSTFLAGS=-Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld
    // cpp.flag_if_supported("-flto");
    cpp.build("src/lib.rs");

    cc::Build::new()
        .cpp(true)
        .includes(qt5core.include_paths)
        .includes(qt5gui.include_paths)
        .includes(qt5qml.include_paths)
        .file("src/ffi/qffi.cpp")
        .compile("qffi");
}
