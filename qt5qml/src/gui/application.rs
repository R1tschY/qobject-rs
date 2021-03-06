use crate::core::QApplicationFactory;
use std::os::raw::c_char;

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

impl QApplicationFactory for QGuiApplication {
    type ApplicationType = Self;

    unsafe fn create_app(argc: *mut i32, argv: *const *const c_char) -> *mut QGuiApplication {
        cpp!([argc as "int*", argv as "char**"] -> *mut QGuiApplication as "QGuiApplication*" {
            return new QGuiApplication(*argc, argv);
        })
    }
}
