#[macro_use]
extern crate cpp;

#[macro_export]
macro_rules! opaque_struct {
    ($x:ident) => {
        #[repr(C)]
        pub struct $x {
            _private: [u8; 0],
        }
    };
}

pub mod core;

use std::ops::Deref;
use std::ptr;

pub trait Deletable {
    unsafe fn delete(&mut self);
}

pub struct QBox<T: Deletable>(ptr::NonNull<T>);

impl<T: Deletable> QBox<T> {
    pub unsafe fn from_raw(ptr: *mut T) -> Self {
        Self(ptr::NonNull::new(ptr).expect("tried to created QBox from null pointer"))
    }
}

impl<T: Deletable> Deref for QBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}

impl<T: Deletable> Drop for QBox<T> {
    fn drop(&mut self) {
        unsafe { T::delete(self.0.as_mut()) };
    }
}
