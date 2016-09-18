extern crate libc;

use libc::{c_char, uint32_t};
use std::ffi::CString;

mod constellation;
mod formatter;
mod bounding_box;
mod color;

#[no_mangle]
pub extern fn extern_constellation_svg(origin_x: uint32_t, origin_y: uint32_t, width: uint32_t, height: uint32_t) -> *mut c_char {
    let constellation = constellation_svg(origin_x as i32, origin_y as i32, width as i32, height as i32);

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

pub fn constellation_svg(origin_x: i32, origin_y: i32, width: i32, height: i32) -> String {
    let bounds = bounding_box::BoundingBox {
        origin_x: origin_x,
        origin_y: origin_y,
        width: width,
        height: height
    };
    let constellation = constellation::Constellation::new(&bounds);
    formatter::svg(constellation, bounds)
}