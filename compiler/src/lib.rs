#[macro_use]
extern crate lazy_static;

pub use cc::Build as CcBuild;

use crate::build::build;
use crate::moc::MocConfig;
use crate::qobject::QObjectConfig;
pub use crate::qobject::{QObjectMethod, QObjectProp, QObjectSignal, TypeRef};
use std::env;
use std::path::PathBuf;
use std::process::exit;

pub mod ffi;
pub mod generate;
pub mod moc;
pub mod qobject;
pub mod rust;

pub mod build;

pub struct QObjectBuild {
    obj: QObjectConfig,
}

impl QObjectBuild {
    pub fn new(name: &str) -> Self {
        Self {
            obj: QObjectConfig::new(name),
        }
    }

    pub fn inherit<T: Into<TypeRef>>(&mut self, type_ref: T) -> &mut Self {
        self.obj.inherit(type_ref.into());
        self
    }

    pub fn property<T: Into<QObjectProp>>(&mut self, prop: T) -> &mut Self {
        self.obj.property(prop.into());
        self
    }

    pub fn method<T: Into<QObjectMethod>>(&mut self, meth: T) -> &mut Self {
        self.obj.method(meth.into());
        self
    }

    pub fn signal<T: Into<QObjectSignal>>(&mut self, signal: T) -> &mut Self {
        self.obj.signal(signal.into());
        self
    }

    pub fn build(&self, cc: &CcBuild, moc: &MocConfig) {
        // TODO: move logic out of here
        let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR environment variable not set");
        let path = PathBuf::from(out_dir).join(format!("qffi_{}.cpp", self.obj.name));
        if let Err(err) = build(cc, moc, &path, &self.obj) {
            eprintln!("Failed to compile C++ class: {:?}", err);
            exit(1);
        }
    }
}
