use crate::core::{QCoreApplication, QCoreApplicationFactory};
use crate::{CppBox, Deletable, QBox};
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::pin::Pin;

cpp! {{
    #include <QGuiApplication>
}}

opaque_struct!(QGuiApplication);
impl_qobject_ref!(QGuiApplication);

impl QGuiApplication {
    pub fn exec(&self) -> i32 {
        cpp!(unsafe [] -> i32 as "int" {
            return QGuiApplication::exec();
        })
    }
}

impl QCoreApplicationFactory for QGuiApplication {
    unsafe fn create_app(argc: *mut i32, argv: *const *const i8) -> *mut QGuiApplication {
        cpp!([argc as "int*", argv as "char**"] -> *mut QGuiApplication as "QGuiApplication*" {
            return new QGuiApplication(*argc, argv);
        })
    }
}