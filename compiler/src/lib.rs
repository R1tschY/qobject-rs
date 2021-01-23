use std::env;
use std::path::PathBuf;
use std::process::exit;

pub use cc::Build as CcBuild;

use crate::build::build;
use crate::moc::MocConfig;
use crate::qobject::QObjectConfig;
pub use crate::qobject::{QObjectMethod, QObjectProp, QObjectSignal};
pub use crate::typeref::{Include, TypeRef, TypeRefTrait};

pub mod ffi;
pub mod generate;
pub mod moc;
pub mod qobject;
pub mod typeref;
mod utils;

pub mod build;

/// Builder for a C++ Qt class.
pub struct QObjectBuild {
    obj: QObjectConfig,
}

impl QObjectBuild {
    /// Construct builder for a class named `name`.
    pub fn new(name: &str) -> Self {
        Self {
            obj: QObjectConfig::new(name),
        }
    }

    /// Set super class.
    ///
    /// Default is `QObject`.
    pub fn inherit<T: Into<TypeRef>>(&mut self, type_ref: T) -> &mut Self {
        self.obj.inherit(type_ref.into());
        self
    }

    /// Add a property.
    pub fn property(&mut self, prop: QObjectProp) -> &mut Self {
        self.obj.property(prop);
        self
    }

    /// Add a method.
    pub fn method(&mut self, meth: QObjectMethod) -> &mut Self {
        self.obj.method(meth);
        self
    }

    /// Add a slot.
    pub fn slot(&mut self, meth: QObjectMethod) -> &mut Self {
        self.obj.slot(meth);
        self
    }

    /// Add a signal.
    pub fn signal(&mut self, signal: QObjectSignal) -> &mut Self {
        self.obj.signal(signal);
        self
    }

    /// Set that QML register function is generated.
    pub fn qml(&mut self, value: bool) -> &mut Self {
        self.obj.qml(value);
        self
    }

    /// Build C++ source file.
    ///
    /// This function generates the C++ source file, generates the moc file and compiles the
    /// sources. The compiled object file is added to the cargo build process.
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
