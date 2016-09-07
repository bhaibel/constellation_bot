extern crate libc;

use libc::{c_char};
use std::ffi::CString;

mod star_field;
mod formatter;

#[no_mangle]
pub extern fn extern_constellation_svg() -> *mut c_char {
    let constellation = constellation_svg();

    let c_str_constellation = CString::new(constellation).unwrap();
    c_str_constellation.into_raw()
}

#[no_mangle]
pub extern fn extern_constellation_free(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        CString::from_raw(s)
    };
}

pub fn constellation_svg() -> String {
    let star_field = star_field::stars();
    formatter::svg(star_field)
}