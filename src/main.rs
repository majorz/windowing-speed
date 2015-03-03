#![feature(libc)]

extern crate libc;

mod ffi;


fn main() {
    let xcb = ffi::xcb::XCB::new();

    xcb.print_dimensions();

    xcb.disconnect();
}
