#![no_std]
mod cstr_core;
extern crate alloc;
use crate::alloc::borrow::ToOwned;
use alloc::string::String;
use core::convert::TryInto;

pub type CString = i32;

pub fn from_str(s: &str) -> CString {
    cstr_core::CString::new(s).unwrap().into_raw() as i32
}

pub fn try_into_string(start: impl TryInto<CString>) -> Result<String, &'static str> {
    if let Ok(pos) = start.try_into() {
        let s: &cstr_core::CStr =
            unsafe { cstr_core::CStr::from_ptr(pos as *const cstr_core::c_char) };
        if let Ok(s) = s.to_str() {
            Ok(s.to_owned())
        } else {
            Err("error creating cstring")
        }
    } else {
        Err("could not decypher cstring starting point")
    }
}
