use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_char, c_int};
use std::pin::Pin;

use crate::core::QObjectRef;
use crate::ffi::{qffi_QCoreApplication_exec, qffi_QCoreApplication_init};
use crate::QBox;

#[repr(C)]
pub struct QCoreApplication(pub(crate) crate::ffi::QCoreApplication);
impl_ffi_trait!(QCoreApplication);
impl_qobject_ref!(QCoreApplication);

impl QCoreApplication {
    pub fn exec(&self) -> i32 {
        unsafe { qffi_QCoreApplication_exec() }
    }
}

pub trait QApplicationFactory {
    type ApplicationType: QObjectRef;

    unsafe fn create_app(argc: *mut c_int, argv: *mut *const c_char) -> *mut Self::ApplicationType;

    fn new_from_env_args() -> QApplicationHolder<Self::ApplicationType>
    where
        Self: std::marker::Sized,
    {
        let mut argv_owned: Pin<Box<[CString]>> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect::<Box<[CString]>>()
            .into();
        let mut argv: Pin<Box<[*const c_char]>> = argv_owned
            .iter_mut()
            .map(|arg| arg.as_ptr())
            .collect::<Box<[*const c_char]>>()
            .into();

        let mut argc: Pin<Box<c_int>> = Box::pin(argv_owned.len() as c_int);
        let mut argv_ptr = argv.as_mut_ptr();
        let argc_ptr: *mut c_int = &mut *argc;
        let app = unsafe { QBox::from_raw(Self::create_app(argc_ptr, argv_ptr)) };

        QApplicationHolder {
            _argv_owned: argv_owned,
            _argv: argv,
            _argc: argc,
            app,
        }
    }
}

impl QApplicationFactory for QCoreApplication {
    type ApplicationType = Self;

    unsafe fn create_app(argc: *mut i32, argv: *mut *const c_char) -> *mut QCoreApplication {
        unsafe { std::mem::transmute(qffi_QCoreApplication_init(argc, argv)) }
    }
}

pub struct QApplicationHolder<T: QObjectRef> {
    _argv_owned: Pin<Box<[CString]>>,
    _argv: Pin<Box<[*const c_char]>>,
    _argc: Pin<Box<c_int>>,
    app: QBox<T>,
}

impl<T: QObjectRef> Deref for QApplicationHolder<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

impl<T: QObjectRef> DerefMut for QApplicationHolder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.app
    }
}
