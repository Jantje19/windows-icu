use std::ops::DerefMut;

use windows::Win32::Globalization::{UEnumeration, ucal_openTimeZones, uenum_close, uenum_next};

use crate::icu::{error::Error, status::ICUStatus};

pub struct TimeZoneIterator {
    ptr: *mut UEnumeration,
}

impl TimeZoneIterator {
    pub fn new() -> Result<Self, Error> {
        let mut status = ICUStatus::new();
        let ptr = unsafe { ucal_openTimeZones(status.deref_mut()) };

        if status.is_error() {
            return Err(Error::InvalidStatus(status));
        }

        if ptr.is_null() {
            return Err(Error::UnexpectedNullPtr);
        }

        Ok(TimeZoneIterator { ptr })
    }
}

impl Iterator for TimeZoneIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut len = 0;
        let mut status = ICUStatus::new();
        let tz_ptr = unsafe { uenum_next(self.ptr, &mut len, status.deref_mut()) };

        if tz_ptr.is_null() {
            None
        } else {
            let bytes = unsafe { std::slice::from_raw_parts(tz_ptr.as_ptr(), len as _) };
            Some(String::from_utf8(bytes.into()).unwrap())
        }
    }
}

impl Drop for TimeZoneIterator {
    fn drop(&mut self) {
        unsafe {
            uenum_close(self.ptr);
        }
    }
}
