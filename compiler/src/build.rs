use crate::generate::generate;
use crate::moc::MocConfig;
use crate::qobject::QObjectConfig;
use crate::CcBuild;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn invalid_arg(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

pub fn build(
    cpp: &CcBuild,
    moc: &MocConfig,
    path: &Path,
    obj: &QObjectConfig,
) -> Result<(), Box<dyn Error>> {
    let moc_path: PathBuf = path.with_extension("moc");
    let rs_path: PathBuf = path.with_extension("rs");
    let output_name = path.file_stem().unwrap().to_str().unwrap();

    let moc_name = moc_path
        .file_name()
        .ok_or_else(|| invalid_arg("input path does not point to a file"))?
        .to_str()
        .ok_or_else(|| invalid_arg("input path is not valid UTF-8"))?;

    // Generate
    let (cpp_code, rust_code) = generate(&moc_name, &[obj]);
    // TODO: check content before writing
    fs::write(path, &cpp_code).unwrap();
    fs::write(rs_path, &rust_code).unwrap();

    // MOC
    moc.build(path, &moc_path).unwrap();

    // C++
    cpp.clone()
        .cpp(true)
        .file(path)
        .static_flag(true)
        .try_compile(output_name)
        .unwrap();

    Ok(())
}
