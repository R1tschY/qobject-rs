// GENERATED -- DO NOT EDIT!!

use crate::core::QByteArray;
use std::collections::HashMap;


cpp! {{
    #include <QHash>
    #include <QByteArray>
    
}}


cpp_class!(
    #[derive(Clone, PartialEq, Eq)]
    pub unsafe struct QHashIntQByteArray as "QHash<int, QByteArray>"
);

impl QHashIntQByteArray {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> i32 {
        cpp!(unsafe [self as "const QHash<int, QByteArray>*"] -> i32 as "int" {
            return self->size();
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, key: i32, value: QByteArray) {
        cpp!(unsafe [self as "QHash<int, QByteArray>*", key as "int", value as "QByteArray"] {
            self->insert(key, value);
        })
    }
}

impl From<HashMap<i32, QByteArray>> for QHashIntQByteArray {
    fn from(value: HashMap<i32, QByteArray>) -> Self {
        let mut result = Self::new();
        for entry in value {
            result.insert(entry.0, entry.1);
        }
        result
    }
}

