use std::{ffi::c_void, ops::DerefMut};

use windows::Win32::Globalization::{
    UCAL_DEFAULT, UCAL_DST_OFFSET, UCAL_ZONE_OFFSET, ucal_close, ucal_get, ucal_open,
    ucal_setMillis,
};

use crate::icu::{error, status::ICUStatus};

pub struct ICUCalendar {
    ptr: *mut *mut c_void,
}

impl ICUCalendar {
    pub fn new(tz_name: &str) -> Result<Self, error::Error> {
        let locale = windows::core::PCSTR::from_raw(c"en_US".as_ptr() as _);

        let mut tzid: Vec<u16> = tz_name.encode_utf16().collect();
        tzid.push(0);

        let mut status = ICUStatus::new();
        let calendar =
            unsafe { ucal_open(tzid.as_ptr(), -1, locale, UCAL_DEFAULT, status.deref_mut()) };

        if status.is_error() {
            return Err(error::Error::InvalidStatus(status));
        }

        if calendar.is_null() {
            return Err(error::Error::UnexpectedNullPtr);
        }

        Ok(Self { ptr: calendar })
    }

    pub fn get_time_zone_offset_in_ms(&mut self, at: f64) -> Result<i32, error::Error> {
        self.set_time(at)?;

        let mut status = ICUStatus::new();
        let zone_offset = unsafe { ucal_get(self.ptr as _, UCAL_ZONE_OFFSET, status.deref_mut()) };
        let dst_offset = unsafe { ucal_get(self.ptr as _, UCAL_DST_OFFSET, status.deref_mut()) };

        if status.is_error() {
            Err(error::Error::InvalidStatus(status))
        } else {
            Ok(zone_offset + dst_offset)
        }
    }

    fn set_time(&mut self, at: f64) -> Result<(), error::Error> {
        let mut status = ICUStatus::new();

        unsafe { ucal_setMillis(self.ptr, at, status.deref_mut()) };

        if status.is_error() {
            Err(error::Error::InvalidStatus(status))
        } else {
            Ok(())
        }
    }
}

impl Drop for ICUCalendar {
    fn drop(&mut self) {
        unsafe { ucal_close(self.ptr) };
    }
}
