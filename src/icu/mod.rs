use std::ops::DerefMut;

use windows::Win32::Globalization::{
    ucal_getNow, ucal_getTZDataVersion, ucal_getWindowsTimeZoneID,
};

use crate::icu::{error::Error, status::ICUStatus};

pub mod calendar;
pub mod error;
pub mod status;
pub mod timezones;

pub fn get_time_zone_data_version() -> Result<String, Error> {
    let mut status = ICUStatus::new();
    let pcstr = unsafe { ucal_getTZDataVersion(status.deref_mut()) };

    if status.is_error() {
        Err(Error::InvalidStatus(status))
    } else {
        Ok(unsafe { pcstr.to_string() }.unwrap())
    }
}

pub fn now() -> f64 {
    unsafe { ucal_getNow() }
}

// https://devblogs.microsoft.com/oldnewthing/20210527-00/?p=105255 (How can I convert between IANA time zones and Windows registry-based time zones?)
pub fn iana_to_windows(tz_name: &str) -> Result<Option<String>, Error> {
    let icu_tz: Vec<u16> = tz_name.encode_utf16().chain(std::iter::once(0)).collect();

    // Output buffer for Windows time zone ID
    let mut windows_tz = [0u16; 128];
    let mut status = ICUStatus::new();

    let result_len = unsafe {
        ucal_getWindowsTimeZoneID(
            icu_tz.as_ptr(),
            -1,
            windows_tz.as_mut_ptr(),
            windows_tz.len() as _,
            status.deref_mut(),
        )
    };

    if status.is_error() {
        return Err(Error::InvalidStatus(status));
    }

    if result_len <= 0 {
        return Ok(None);
    }

    let data = &windows_tz[..result_len as usize];

    Ok(Some(String::from_utf16_lossy(data)))
}
