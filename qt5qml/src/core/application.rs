use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::os::raw::{c_char, c_int};
use std::pin::Pin;

use crate::core::QObjectRef;
use crate::{CppBox, Deletable, QBox};

cpp! {{
    #include <QCoreApplication>
}}

opaque_struct!(QCoreApplication);
impl_qobject_ref!(QCoreApplication);

impl QCoreApplication {
    pub fn exec(&self) -> i32 {
        cpp!(unsafe [] -> i32 as "int" {
            return QCoreApplication::exec();
        })
    }
}

fn pin_vec<T>(input: Vec<T>) -> Pin<Vec<T>> {
    unsafe { Pin::new_unchecked(input) }
}

pub trait QApplicationFactory: QObjectRef {
    unsafe fn create_app(argc: *mut c_int, argv: *const *const c_char) -> *mut Self;

    fn new_from_env_args() -> QApplicationHolder<Self>
    where
        Self: std::marker::Sized,
    {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();

        let argv_owned: Pin<Vec<CString>> = pin_vec(args);
        let argv: Pin<Vec<*const c_char>> =
            pin_vec(argv_owned.iter().map(|arg| arg.as_ptr()).collect());
        let mut argc: Pin<Box<c_int>> = Box::pin(argv_owned.len() as c_int);
        let argv_ptr = argv.as_ptr();
        let argc_ptr: *mut c_int = &mut *argc;
        let app = unsafe { QBox::from_raw(Self::create_app(argc_ptr, argv_ptr)) };

        QApplicationHolder {
            argv_owned,
            argv,
            argc,
            app,
        }
    }
}

impl QApplicationFactory for QCoreApplication {
    unsafe fn create_app(argc: *mut i32, argv: *const *const c_char) -> *mut QCoreApplication {
        cpp!([argc as "int*", argv as "char**"] -> *mut QCoreApplication as "QCoreApplication*" {
            return new QCoreApplication(*argc, argv);
        })
    }
}

pub struct QApplicationHolder<T: QObjectRef> {
    argv_owned: Pin<Vec<CString>>,
    argv: Pin<Vec<*const c_char>>,
    argc: Pin<Box<c_int>>,
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
