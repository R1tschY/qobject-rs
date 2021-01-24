#![allow(unused)]

include!(concat!(env!("OUT_DIR"), "/qffi_TestObject.rs"));

pub struct TestObjectPrivate {
    qobject: *mut TestObject,
}

impl TestObjectPrivate {
    pub fn new(qobject: *mut TestObject) -> Self {
        Self { qobject }
    }

    pub fn emit_signal_0(&mut self) {
        unsafe { (&mut *self.qobject).signal0() };
    }
}

#[test]
fn test_signal() {
    use std::ptr;

    let mut object = TestObject::new();
    object.signal0();
}
