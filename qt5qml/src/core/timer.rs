use std::ptr;
use std::time::Duration;

use crate::core::{QObject, Signal, Slot};
use crate::ffi::QffiWrapper;
use crate::QBox;

#[repr(C)]
pub struct QTimer(pub(crate) crate::ffi::QTimer);
impl_ffi_trait!(QTimer);
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
        unsafe {
            QBox::from_raw(std::mem::transmute(crate::ffi::qffi_QTimer_init(
                ptr::null_mut(),
            )))
        }
    }

    #[inline]
    pub fn new_with_parent(parent: &mut QObject) -> *mut QTimer {
        unsafe { std::mem::transmute(crate::ffi::qffi_QTimer_init(parent.to_inner_mut())) }
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        unsafe { crate::ffi::qffi_QTimer_isActive(&self.0) }
    }

    #[inline]
    pub fn interval(&self) -> Duration {
        Duration::from_millis(unsafe { crate::ffi::qffi_QTimer_interval(&self.0) } as u64)
    }

    #[inline]
    pub fn set_interval(&mut self, duration: Duration) {
        unsafe { crate::ffi::qffi_QTimer_setInterval(&mut self.0, duration.as_millis() as i32) }
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        let result = unsafe { crate::ffi::qffi_QTimer_remainingTime(&self.0) };
        if result < 0 {
            None
        } else {
            Some(Duration::from_millis(result as u64))
        }
    }

    #[inline]
    pub fn is_single_shot(&self) -> bool {
        unsafe { crate::ffi::qffi_QTimer_isSingleShot(&self.0) }
    }

    #[inline]
    pub fn set_single_shot(&mut self, single_shot: bool) {
        unsafe { crate::ffi::qffi_QTimer_setSingleShot(&mut self.0, single_shot) }
    }

    #[inline]
    pub fn timer_type(&self) -> TimerType {
        unsafe { crate::ffi::qffi_QTimer_timerType(&self.0) }.into()
    }

    #[inline]
    pub fn set_timer_type(&mut self, timer_type: TimerType) {
        unsafe { crate::ffi::qffi_QTimer_setTimerType(&mut self.0, timer_type.into()) }
    }

    #[inline]
    pub fn start(&mut self) {
        unsafe { crate::ffi::qffi_QTimer_start(&mut self.0) }
    }

    pub fn start_with_interval(&mut self, duration: Duration) {
        unsafe {
            crate::ffi::qffi_QTimer_startWithInterval(&mut self.0, duration.as_millis() as i32)
        }
    }

    #[inline]
    pub fn stop(&mut self) {
        unsafe { crate::ffi::qffi_QTimer_stop(&mut self.0) }
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
