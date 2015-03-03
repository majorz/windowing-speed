use libc::{c_int, c_char};
use std::ptr;

#[repr(C)]
struct XcbConnectionFFI;

#[link(name = "xcb")]
extern {
	fn xcb_connect(displayname: *const c_char, screenp:*mut c_int) -> *mut XcbConnectionFFI;
	fn xcb_connection_has_error(c: *mut XcbConnectionFFI) -> c_int;
	fn xcb_disconnect(c: *mut XcbConnectionFFI);
}


pub struct Connection {
    c: *mut XcbConnectionFFI,
}


impl Connection {
	#[inline]
	pub fn new() -> Self {
		let mut screen: c_int = 0;
		unsafe {
			let c = xcb_connect(ptr::null(), &mut screen);
			if xcb_connection_has_error(c) > 0 {
				panic!("A XCB connection was not established due to a fatal error.")
			}

			Connection{
				c: c,
			}
        }
	}

	pub fn disconnect(&self) {
		unsafe {
			xcb_disconnect(self.c);
        }
	}
}
