extern crate libc;

use libc::{c_char, uint8_t};
use std::ffi::CString;
use std::iter;

#[no_mangle]
pub extern fn constellation_generate() -> *mut c_char {
    let mut constellation = String::from("<svg>");
    constellation.push_str("</svg>");

    let c_str_constellation = CString::new(constellation).unwrap();
    c_str_constellation.into_raw()
}

#[no_mangle]
pub extern fn constellation_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}
