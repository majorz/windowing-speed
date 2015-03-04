#![feature(libc)]

extern crate libc;

mod ffi;


fn main() {
    let mut xcb = ffi::xcb::XCB::new();

    xcb.create_window();

    xcb.print_screen_dimensions();

    xcb.pause();

    xcb.disconnect();
}
