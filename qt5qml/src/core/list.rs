// GENERATED -- DO NOT EDIT!!

use std::iter::FromIterator;

cpp! {{
    #include <QList>
    #include <QObject>
    #include <QString>
    
}}


cpp_class!(
    #[derive(Clone, PartialEq, Eq)]
    pub unsafe struct QObjectList as "QList<QObject*>"
);

impl QObjectList {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> i32 {
        cpp!(unsafe [self as "const QList<QObject*>*"] -> i32 as "int" {
            return self->size();
        })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[*mut crate::core::QObject] {
        let mut size = 0;
        let ptr = cpp!(unsafe [
            self as "const QList<QObject*>*", mut size as "size_t"
        ] -> *const *mut crate::core::QObject as "QObject* const*" {
            size = self->size();
            return &self->front();
        });
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn push(&mut self, item: *mut crate::core::QObject) {
        cpp!(unsafe [self as "QList<QObject*>*", item as "QObject*"] {
            self->append(item);
        })
    }

    pub fn append(&mut self, value: &QObjectList) {
        cpp!(unsafe [self as "QList<QObject*>*",
                     value as "const QList<QObject*>*"] {
            self->append(*value);
        })
    }

    pub fn extend_from_slice(&mut self, slice: &[*mut crate::core::QObject]) {
        let ptr = slice.as_ptr();
        let size = slice.len();
        cpp!(unsafe [self as "QList<QObject*>*", ptr as "QObject* const*", size as "size_t"] {
            self->reserve(self->size() + size);
            for (size_t i = 0; i < size; ++i) {
                self->push_back(ptr[i]);
            }
        })
    }

    pub fn reserve(&self, additional: usize) {
        cpp!(unsafe [self as "QList<QObject*>*", additional as "size_t"] {
            self->reserve(self->size() + additional);
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &*mut crate::core::QObject> {
        self.as_slice().iter()
    }
}

impl Extend<*mut crate::core::QObject> for QObjectList {
    fn extend<T: IntoIterator<Item = *mut crate::core::QObject>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl FromIterator<*mut crate::core::QObject> for QObjectList {
    fn from_iter<T: IntoIterator<Item = *mut crate::core::QObject>>(iter: T) -> Self {
        let mut res = Self::default();
        res.extend(iter);
        res
    }
}

impl<'a> IntoIterator for &'a QObjectList {
    type Item = &'a *mut crate::core::QObject;
    type IntoIter = std::slice::Iter<'a, *mut crate::core::QObject>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

cpp_class!(
    #[derive(Clone, PartialEq, Eq)]
    pub unsafe struct QStringList as "QList<QString>"
);

impl QStringList {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn len(&self) -> i32 {
        cpp!(unsafe [self as "const QList<QString>*"] -> i32 as "int" {
            return self->size();
        })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[crate::core::QString] {
        let mut size = 0;
        let ptr = cpp!(unsafe [
            self as "const QList<QString>*", mut size as "size_t"
        ] -> *const crate::core::QString as "QString const*" {
            size = self->size();
            return &self->front();
        });
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }

    pub fn push(&mut self, item: crate::core::QString) {
        cpp!(unsafe [self as "QList<QString>*", item as "QString"] {
            self->append(item);
        })
    }

    pub fn append(&mut self, value: &QStringList) {
        cpp!(unsafe [self as "QList<QString>*",
                     value as "const QList<QString>*"] {
            self->append(*value);
        })
    }

    pub fn extend_from_slice(&mut self, slice: &[crate::core::QString]) {
        let ptr = slice.as_ptr();
        let size = slice.len();
        cpp!(unsafe [self as "QList<QString>*", ptr as "QString const*", size as "size_t"] {
            self->reserve(self->size() + size);
            for (size_t i = 0; i < size; ++i) {
                self->push_back(ptr[i]);
            }
        })
    }

    pub fn reserve(&self, additional: usize) {
        cpp!(unsafe [self as "QList<QString>*", additional as "size_t"] {
            self->reserve(self->size() + additional);
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = &crate::core::QString> {
        self.as_slice().iter()
    }
}

impl Extend<crate::core::QString> for QStringList {
    fn extend<T: IntoIterator<Item = crate::core::QString>>(&mut self, iter: T) {
        for item in iter {
            self.push(item);
        }
    }
}

impl FromIterator<crate::core::QString> for QStringList {
    fn from_iter<T: IntoIterator<Item = crate::core::QString>>(iter: T) -> Self {
        let mut res = Self::default();
        res.extend(iter);
        res
    }
}

impl<'a> IntoIterator for &'a QStringList {
    type Item = &'a crate::core::QString;
    type IntoIter = std::slice::Iter<'a, crate::core::QString>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

