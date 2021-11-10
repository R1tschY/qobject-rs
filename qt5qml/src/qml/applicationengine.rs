use crate::core::{QObject, QObjectList, QUrl};
use crate::ffi::QffiWrapper;
use crate::QBox;
use std::ptr;

#[repr(C)]
pub struct QQmlApplicationEngine(pub(crate) crate::ffi::QQmlApplicationEngine);
impl_ffi_trait!(QQmlApplicationEngine);
impl_qobject_ref!(QQmlApplicationEngine);

impl QQmlApplicationEngine {
    pub fn new() -> QBox<QQmlApplicationEngine> {
        unsafe {
            QBox::from_raw(std::mem::transmute(
                crate::ffi::qffi_QQmlApplicationEngine_init(ptr::null_mut()),
            ))
        }
    }

    pub fn new_with_parent(parent: &mut QObject) -> *mut QQmlApplicationEngine {
        unsafe {
            std::mem::transmute(crate::ffi::qffi_QQmlApplicationEngine_init(
                parent.to_inner_mut(),
            ))
        }
    }

    fn load_intern(&mut self, url: &QUrl) {
        unsafe {
            crate::ffi::qffi_QQmlApplicationEngine_load(self.to_inner_mut(), url.to_inner());
        }
    }

    pub fn load<T: Into<QUrl>>(&mut self, url: T) {
        self.load_intern(&url.into())
    }

    pub fn load_path(&mut self, file_path: &str) {
        self.load_intern(&QUrl::from_local_file(file_path))
    }

    pub fn root_objects(&self) -> QObjectList {
        unsafe {
            QObjectList::create(|res| {
                crate::ffi::qffi_QQmlApplicationEngine_rootObjects(self.to_inner(), res);
            })
        }
    }
}
