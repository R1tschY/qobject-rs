use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, io};

use crate::generate::generate;
use crate::moc::MocConfig;
use crate::qobject::QObjectConfig;
use crate::CcBuild;

fn invalid_arg(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

fn write_when_changed(path: &Path, content: &str) -> io::Result<bool> {
    let old_content = match fs::read_to_string(path) {
        Ok(old_content) => Some(old_content),
        Err(err) if err.kind() == io::ErrorKind::NotFound => None,
        Err(err) => return Err(err),
    };

    if old_content.as_ref().map(|c| c as &str) != Some(content) {
        fs::write(path, content)?;
        Ok(true)
    } else {
        Ok(false)
    }
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
    let cpp_changed = write_when_changed(path, &cpp_code)?;
    write_when_changed(&rs_path, &rust_code)?;

    if cpp_changed {
        // MOC
        moc.build(path, &moc_path)?;
    }

    // C++
    cpp.clone()
        .cpp(true)
        .file(path)
        .static_flag(true)
        .try_compile(output_name)?;

    Ok(())
}
