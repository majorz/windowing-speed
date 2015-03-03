#![feature(libc)]

extern crate libc;

mod ffi;


fn main() {
    let connection = ffi::xcb::Connection::new();

    println!("Connected!");

    connection.disconnect();
}
