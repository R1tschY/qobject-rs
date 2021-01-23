use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

#[derive(Clone)]
pub struct MocConfig {
    moc: Option<PathBuf>,
    include_paths: Vec<PathBuf>,
    frameworks: Vec<String>,
    macros: HashMap<String, Option<String>>,
    undefs: Vec<String>,
    metadata: HashMap<String, String>,
    compiler_flavor: Option<String>,
    path_prefix: Option<PathBuf>,
    override_include: Option<PathBuf>,
    extra_includes: Vec<PathBuf>,
    includes: Vec<PathBuf>,
}

impl MocConfig {
    pub fn new() -> Self {
        Self {
            moc: None,
            include_paths: vec![],
            frameworks: vec![],
            macros: Default::default(),
            undefs: vec![],
            metadata: Default::default(),
            compiler_flavor: None,
            path_prefix: None,
            override_include: None,
            extra_includes: vec![],
            includes: vec![],
        }
    }

    pub fn include_path(&mut self, path: &Path) -> &mut Self {
        self.include_paths.push(path.into());
        self
    }

    fn get_moc(&self) -> PathBuf {
        match &self.moc {
            Some(moc) => moc.clone(),
            None => match env::var_os("MOC") {
                Some(moc) => moc.into(),
                None => "moc".into(),
            },
        }
    }

    pub fn build(&self, input: &Path, output: &Path) -> io::Result<()> {
        let mut command = Command::new(self.get_moc());
        command.arg("-o").arg(output).arg(input);
        for include in &self.include_paths {
            command.arg("-I").arg(include);
        }

        println!("exec: {:?}", command);
        let result = command.output()?;
        if result.status.success() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                String::from_utf8_lossy(&result.stderr),
            ))
        }
    }
}

impl Default for MocConfig {
    fn default() -> Self {
        Self::new()
    }
}
