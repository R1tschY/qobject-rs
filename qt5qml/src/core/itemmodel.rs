use std::ffi::c_void;
use std::ptr;

cpp! {{
    #include <QAbstractItemModel>
}}

opaque_struct!(QAbstractItemModel);

#[repr(C)]
pub struct QModelIndex {
    r: i32,
    c: i32,
    i: *mut c_void,
    m: *const QAbstractItemModel,
}

impl QModelIndex {
    pub fn new() -> Self {
        Self {
            r: -1,
            c: -1,
            i: ptr::null_mut(),
            m: ptr::null(),
        }
    }

    pub fn row(&self) -> i32 {
        self.r
    }

    pub fn column(&self) -> i32 {
        self.c
    }

    pub fn internal_id(&self) -> usize {
        self.i as usize
    }

    pub fn internal_pointer(&self) -> *mut c_void {
        self.i
    }

    pub fn model(&self) -> *const QAbstractItemModel {
        self.m
    }

    pub fn is_valid(&self) -> bool {
        self.r >= 0 && self.c >= 0 && self.m != ptr::null()
    }

    /*    pub fn parent(&self) -> QModelIndex {
        if self.m != ptr::null() {
            unsafe { (&*m).parent(self) }
        } else {
            QModelIndex::new()
        }
    }

    pub fn sibling(&self, arow: i32, acolumn: i32) -> QModelIndex {
        if self.m != ptr::null() {
            if self.r == arow && self.c == acolumn {
                self.clone()
            } else {
                unsafe { (&*m).sibling(arow, acolumn, self) }
            }
        } else {
            QModelIndex::new()
        }
    }

    pub fn child(&self, arow: i32, acolumn: i32) -> QModelIndex {
        if self.m != ptr::null() {
            unsafe { (&*m).index(arow, acolumn, self) }
        } else {
            QModelIndex::new()
        }
    }

    pub fn data(&self, arole: i32) -> QModelIndex {
        if self.m != ptr::null() {
            unsafe { (&*m).data(self, arole) }
        } else {
            QModelIndex::new()
        }
    }

    pub fn flags(&self) -> ItemFlags {
        if self.m != ptr::null() {
            unsafe { (&*m).flags(self) }
        } else {
            ItemFlags::default()
        }
    }*/
}
