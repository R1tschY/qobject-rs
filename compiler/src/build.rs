use crate::generate::generate_translation_unit;
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
    let output_name = path.file_stem().unwrap().to_str().unwrap();

    let moc_name = moc_path
        .file_name()
        .ok_or_else(|| invalid_arg("input path does not point to a file"))?
        .to_str()
        .ok_or_else(|| invalid_arg("input path is not valid UTF-8"))?;

    // Generate
    let code = generate_translation_unit(&moc_name, &[obj]);
    fs::write(path, &code).unwrap();

    // MOC
    moc.build(path, &moc_path).unwrap();

    // C++
    cpp.clone()
        .file(path)
        .static_flag(true)
        .try_compile(output_name)
        .unwrap();

    Ok(())
}
