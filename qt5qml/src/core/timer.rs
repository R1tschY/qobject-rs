use crate::core::{QObject, Slot};
use crate::QBox;
use std::ptr;
use std::time::Duration;

cpp! {{
    #include <QTimer>
}}

opaque_struct!(QTimer);
impl_qobject_ref!(QTimer);

#[repr(C)]
pub enum TimerType {
    Precise = 0,
    Coarse = 1,
    VeryCoarse = 2,
}

impl From<i32> for TimerType {
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
    fn from(value: TimerType) -> Self {
        value as i32
    }
}

impl QTimer {
    pub fn new(parent: Option<&mut QObject>) -> QBox<QTimer> {
        let parent: *mut QObject = parent.map_or(ptr::null_mut(), |p| p as *mut QObject);
        unsafe {
            QBox::from_raw(
                cpp!(unsafe [parent as "QObject*"] -> *mut QTimer as "QTimer*" {
                    return new QTimer(parent);
                }),
            )
        }
    }

    pub fn is_active(&self) -> bool {
        cpp!(unsafe [self as "const QTimer*"] -> bool as "bool" {
            return self->isActive();
        })
    }

    pub fn interval(&self) -> Duration {
        Duration::from_millis(cpp!(unsafe [self as "const QTimer*"] -> i32 as "int" {
            return self->interval();
        }) as u64)
    }

    pub fn set_interval(&mut self, duration: Duration) {
        let duration: i32 = duration.as_millis() as i32;
        cpp!(unsafe [self as "QTimer*", duration as "int"] {
            self->setInterval(duration);
        });
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        let result = cpp!(unsafe [self as "const QTimer*"] -> i32 as "int" {
            return self->remainingTime();
        });
        if result < 0 {
            None
        } else {
            Some(Duration::from_millis(result as u64))
        }
    }

    pub fn is_single_shot(&self) -> bool {
        cpp!(unsafe [self as "const QTimer*"] -> bool as "bool" {
            return self->isSingleShot();
        })
    }

    pub fn set_single_shot(&mut self, single_shot: bool) {
        cpp!(unsafe [self as "QTimer*", single_shot as "bool"] {
            self->setSingleShot(single_shot);
        });
    }

    pub fn timer_type(&self) -> TimerType {
        cpp!(unsafe [self as "const QTimer*"] -> i32 as "int" {
            return self->timerType();
        })
        .into()
    }

    pub fn set_timer_type(&mut self, timer_type: TimerType) {
        let timer_type: i32 = timer_type.into();
        cpp!(unsafe [self as "QTimer*", timer_type as "int"] {
            self->setTimerType(static_cast<Qt::TimerType>(timer_type));
        });
    }

    pub fn start_slot() -> Slot {
        slot!("start()")
    }

    /// Note: Private signal
    pub fn timeout_signal() -> Slot {
        slot!("start()")
    }
}
