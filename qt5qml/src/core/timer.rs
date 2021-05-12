use std::ptr;
use std::time::Duration;

use crate::core::{QObject, Signal, Slot};
pub use crate::ffi::QTimer;
use crate::QBox;

impl_qobject_ref!(QTimer);

#[repr(C)]
pub enum TimerType {
    Precise = 0,
    Coarse = 1,
    VeryCoarse = 2,
}

impl From<i32> for TimerType {
    #[inline]
    fn from(value: i32) -> Self {
        use TimerType::*;
        match value {
            0 => Precise,
            1 => Coarse,
            2 => VeryCoarse,
            _ => panic!("unknown TimerType {}", value),
        }
    }
}

impl From<TimerType> for i32 {
    #[inline]
    fn from(value: TimerType) -> Self {
        value as i32
    }
}

impl QTimer {
    #[inline]
    pub fn new() -> QBox<QTimer> {
        unsafe { QBox::from_raw(crate::ffi::qffi_QTimer_init(ptr::null_mut())) }
    }

    #[inline]
    pub fn new_with_parent(parent: &mut QObject) -> *mut QTimer {
        unsafe { crate::ffi::qffi_QTimer_init(parent) }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { crate::ffi::qffi_QTimer_isActive(self) }
    }

    #[inline]
    pub fn interval(&self) -> Duration {
        Duration::from_millis(unsafe { crate::ffi::qffi_QTimer_interval(self) } as u64)
    }

    #[inline]
    pub fn set_interval(&mut self, duration: Duration) {
        unsafe { crate::ffi::qffi_QTimer_setInterval(self, duration.as_millis() as i32) }
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        let result = unsafe { crate::ffi::qffi_QTimer_remainingTime(self) };
        if result < 0 {
            None
        } else {
            Some(Duration::from_millis(result as u64))
        }
    }

    #[inline]
    pub fn is_single_shot(&self) -> bool {
        unsafe { crate::ffi::qffi_QTimer_isSingleShot(self) }
    }

    #[inline]
    pub fn set_single_shot(&mut self, single_shot: bool) {
        unsafe { crate::ffi::qffi_QTimer_setSingleShot(self, single_shot) }
    }

    #[inline]
    pub fn timer_type(&self) -> TimerType {
        unsafe { crate::ffi::qffi_QTimer_timerType(self) }.into()
    }

    #[inline]
    pub fn set_timer_type(&mut self, timer_type: TimerType) {
        unsafe { crate::ffi::qffi_QTimer_setTimerType(self, timer_type.into()) }
    }

    #[inline]
    pub fn start(&mut self) {
        unsafe { crate::ffi::qffi_QTimer_start(self) }
    }

    pub fn start_with_interval(&mut self, duration: Duration) {
        unsafe { crate::ffi::qffi_QTimer_startWithInterval(self, duration.as_millis() as i32) }
    }

    #[inline]
    pub fn stop(&mut self) {
        unsafe { crate::ffi::qffi_QTimer_stop(self) }
    }

    pub fn start_slot() -> Slot {
        slot!("start()")
    }

    pub fn stop_slot() -> Slot {
        slot!("stop()")
    }

    /// Note: Private signal
    pub fn timeout_signal() -> Signal {
        signal!("timeout()")
    }
}
