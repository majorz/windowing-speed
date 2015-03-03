#![feature(libc)]

extern crate libc;

mod ffi;


fn main() {
    let connection = ffi::xcb::Connection::new();

    let screen = connection.screen();

    screen.print_dimensions();

    connection.disconnect();
}
