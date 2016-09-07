extern crate libc;
extern crate svg;

use libc::{c_char};
use std::ffi::CString;
use svg::Document;

mod star_set;

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
    Document::new()
        .set("viewBox", (0, 0, 100, 100))
        .set("width", 100)
        .set("height", 100)
        .to_string()
}