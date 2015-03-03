use libc::{c_int, c_char, c_uchar, c_ushort, c_uint};
use std::ptr;

// TYPES & STRUCTURES

type XCBKeycodeFFI = c_uchar;

type XCBWindowFFI = c_uint;

type XCBColormapFFI = c_uint;

type XCBVisualidFFI = c_uint;

#[repr(C)]
struct XCBConnectionFFI;

#[repr(C)]
struct XCBSetupFFI {
    status: c_uchar,
    pad0: c_uchar,
    protocol_major_version: c_ushort,
    protocol_minor_version: c_ushort,
    length: c_ushort,
    release_number: c_uint,
    resource_id_base: c_uint,
    resource_id_mask: c_uint,
    motion_buffer_size: c_uint,
    vendor_len: c_ushort,
    maximum_request_length: c_ushort,
    roots_len: c_uchar,
    pixmap_formats_len: c_uchar,
    image_byte_order: c_uchar,
    bitmap_format_bit_order: c_uchar,
    bitmap_format_scanline_unit: c_uchar,
    bitmap_format_scanline_pad: c_uchar,
    min_keycode: XCBKeycodeFFI,
    max_keycode: XCBKeycodeFFI,
    pad1: [c_uchar; 4usize],
}

#[repr(C)]
struct XCBScreenFFI {
    root: XCBWindowFFI,
    default_colormap: XCBColormapFFI,
    white_pixel: c_uint,
    black_pixel: c_uint,
    current_input_masks: c_uint,
    width_in_pixels: c_ushort,
    height_in_pixels: c_ushort,
    width_in_millimeters: c_ushort,
    height_in_millimeters: c_ushort,
    min_installed_maps: c_ushort,
    max_installed_maps: c_ushort,
    root_visual: XCBVisualidFFI,
    backing_stores: c_uchar,
    save_unders: c_uchar,
    root_depth: c_uchar,
    allowed_depths_len: c_uchar,
}

#[repr(C)]
struct XCBScreenIteratorFFI {
    data: *mut XCBScreenFFI,
    rem: c_int,
    index: c_int,
}

// FUNCTIONS

#[link(name = "xcb")]
extern {
	fn xcb_connect(displayname: *const c_char, screenp:*mut c_int) -> *mut XCBConnectionFFI;
	fn xcb_connection_has_error(c: *mut XCBConnectionFFI) -> c_int;
	fn xcb_disconnect(c: *mut XCBConnectionFFI);

	fn xcb_setup_roots_iterator(R: *mut XCBSetupFFI) -> XCBScreenIteratorFFI;
}


// PUB API

pub struct Connection {
    c: *mut XCBConnectionFFI,
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
