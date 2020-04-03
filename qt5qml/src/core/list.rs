// GENERATED -- DO NOT EDIT!!

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> i32 {
        cpp!(unsafe [self as "const QList<QObject*>*"] -> i32 as "int" {
            return self->size();
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

cpp_class!(
    #[derive(Clone, PartialEq, Eq)]
    pub unsafe struct QStringList as "QList<QString>"
);

impl QStringList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> i32 {
        cpp!(unsafe [self as "const QList<QString>*"] -> i32 as "int" {
            return self->size();
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

