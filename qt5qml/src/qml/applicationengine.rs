use crate::core::{QObject, QObjectList, QUrl};
use crate::QBox;
use std::ptr;

cpp! {{
    #include <QQmlApplicationEngine>
}}

opaque_struct!(QQmlApplicationEngine);
impl_qobject_ref!(QQmlApplicationEngine);

impl QQmlApplicationEngine {
    pub fn new(parent: Option<&mut QObject>) -> QBox<QQmlApplicationEngine> {
        let parent: *mut QObject = parent.map_or(ptr::null_mut(), |p| p as *mut QObject);
        unsafe {
            QBox::from_raw(cpp!(unsafe [parent as "QObject*"]
                    -> *mut QQmlApplicationEngine as "QQmlApplicationEngine*" {
                return new QQmlApplicationEngine(parent);
            }))
        }
    }

    fn load_intern(&mut self, url: &QUrl) {
        cpp!(unsafe [self as "QQmlApplicationEngine*", url as "const QUrl*"] {
            self->load(*url);
        })
    }

    pub fn load<T: Into<QUrl>>(&mut self, url: T) {
        self.load_intern(&url.into())
    }

    pub fn load_path(&mut self, file_path: &str) {
        self.load_intern(&QUrl::from_local_file(file_path))
    }

    pub fn root_objects(&self) -> QObjectList {
        cpp!(unsafe [self as "QQmlApplicationEngine*"] -> QObjectList as "QList<QObject*>" {
            return self->rootObjects();
        })
    }
}
