#[cfg(test)]
#[macro_use]
extern crate indoc;

#[macro_use]
extern crate lazy_static;

pub mod ffi;
pub mod generate;
pub mod moc;
pub mod qobject;

pub mod build;

pub use cc::Build as CcBuild;
