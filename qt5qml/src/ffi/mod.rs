use std::mem::MaybeUninit;

pub(crate) use qffi::*;
pub use qffi::{QObject, QTimer, QUrl};

mod qffi;
mod qffi_impl;

pub(crate) fn init_ffi_struct<T, F>(f: F) -> T
where
    F: Fn(*mut T) -> (),
{
    unsafe {
        let mut ret = MaybeUninit::uninit();
        f(ret.as_mut_ptr());
        ret.assume_init()
    }
}
