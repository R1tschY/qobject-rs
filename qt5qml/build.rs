fn main() {
    let qt5core = pkg_config::probe_library("Qt5Core").unwrap();

    let mut cpp = cpp_build::Config::new();
    for include in &qt5core.include_paths {
        cpp.include(include);
    }
    cpp.build("src/lib.rs");
}
