use crate::core::QString;
use std::fmt;
cpp! {{
    #include <QUrl>
}}

cpp_class!(
    #[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub unsafe struct QUrl as "QUrl"
);

impl QUrl {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_qstring(value: &QString) -> Self {
        cpp!(unsafe [value as "const QString*"] -> QUrl as "QUrl" {
            return QUrl(*value);
        })
    }

    pub fn from_local_file_intern(local_file: &QString) -> Self {
        cpp!(unsafe [local_file as "const QString*"] -> QUrl as "QUrl" {
            return QUrl::fromLocalFile(*local_file);
        })
    }

    pub fn from_local_file<T: Into<QString>>(local_file: T) -> Self {
        Self::from_local_file_intern(&local_file.into())
    }
}

impl From<QString> for QUrl {
    fn from(value: QString) -> Self {
        Self::from_qstring(&value)
    }
}

impl<'a> From<&'a str> for QUrl {
    fn from(value: &'a str) -> Self {
        let value: QString = value.into();
        Self::from_qstring(&value)
    }
}

impl From<String> for QUrl {
    fn from(value: String) -> Self {
        let value: QString = value.into();
        Self::from_qstring(&value)
    }
}

impl fmt::Debug for QUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: String = cpp!(unsafe [self as "const QUrl*"] -> QString as "QString" {
            QString buffer;
            QDebug(&buffer).nospace() << *self;
            return buffer;
        })
        .into();
        f.write_str(&str)
    }
}
