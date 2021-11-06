use std::mem::MaybeUninit;

pub(crate) use qffi::*;
pub use qffi::{QObject, QTimer, QUrl};

mod qffi;
mod qffi_impl;

#[inline]
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

pub(crate) trait QffiWrapper: Sized {
    type QffiObject;

    fn create_from_ffi_object(value: Self::QffiObject) -> Self;
    fn to_inner(&self) -> &Self::QffiObject;
    fn to_inner_mut(&mut self) -> &mut Self::QffiObject;

    #[inline]
    fn create<F>(f: F) -> Self
    where
        F: Fn(*mut Self::QffiObject) -> (),
    {
        unsafe {
            let mut ret = MaybeUninit::uninit();
            f(ret.as_mut_ptr());
            Self::create_from_ffi_object(ret.assume_init())
        }
    }
}
