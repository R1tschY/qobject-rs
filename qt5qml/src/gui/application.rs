use crate::core::QApplicationFactory;
use crate::ffi::{qffi_QGuiApplication_exec, qffi_QGuiApplication_init};
use std::os::raw::c_char;

#[repr(C)]
pub struct QGuiApplication(pub(crate) crate::ffi::QGuiApplication);
impl_qobject_ref!(QGuiApplication);

impl QGuiApplication {
    pub fn exec(&self) -> i32 {
        unsafe { qffi_QGuiApplication_exec() }
    }
}

impl QApplicationFactory for QGuiApplication {
    type ApplicationType = Self;

    unsafe fn create_app(argc: *mut i32, argv: *mut *const c_char) -> *mut QGuiApplication {
        unsafe { std::mem::transmute(qffi_QGuiApplication_init(argc, argv)) }
    }
}
