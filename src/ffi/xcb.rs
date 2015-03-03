#![allow(dead_code)]

use libc::{c_int, c_char, c_uchar, c_ushort, c_uint};
use std::ptr;
use std::mem;
use std::default::Default;

// CONSTANTS

const XCB_GC_FUNCTION: c_uint = 1;
const XCB_GC_PLANE_MASK: c_uint = 2;
const XCB_GC_FOREGROUND: c_uint = 4;
const XCB_GC_BACKGROUND: c_uint = 8;
const XCB_GC_LINE_WIDTH: c_uint = 16;
const XCB_GC_LINE_STYLE: c_uint = 32;
const XCB_GC_CAP_STYLE: c_uint = 64;
const XCB_GC_JOIN_STYLE: c_uint = 128;
const XCB_GC_FILL_STYLE: c_uint = 256;
const XCB_GC_FILL_RULE: c_uint = 512;
const XCB_GC_TILE: c_uint = 1024;
const XCB_GC_STIPPLE: c_uint = 2048;
const XCB_GC_TILE_STIPPLE_ORIGIN_X: c_uint = 4096;
const XCB_GC_TILE_STIPPLE_ORIGIN_Y: c_uint = 8192;
const XCB_GC_FONT: c_uint = 16384;
const XCB_GC_SUBWINDOW_MODE: c_uint = 32768;
const XCB_GC_GRAPHICS_EXPOSURES: c_uint = 65536;
const XCB_GC_CLIP_ORIGIN_X: c_uint = 131072;
const XCB_GC_CLIP_ORIGIN_Y: c_uint = 262144;
const XCB_GC_CLIP_MASK: c_uint = 524288;
const XCB_GC_DASH_OFFSET: c_uint = 1048576;
const XCB_GC_DASH_LIST: c_uint = 2097152;
const XCB_GC_ARC_MODE: c_uint = 4194304;


// TYPES & STRUCTURES

type XCBKeycodeFFI = c_uchar;

type XCBWindowFFI = c_uint;

type XCBColormapFFI = c_uint;

type XCBVisualidFFI = c_uint;

type XCBGcontextFFI = c_uint;

type XCBDrawableFFI = c_uint;

#[repr(C)]
struct XCBConnectionFFI;

#[repr(C)]
#[derive(Copy)]
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
#[derive(Copy)]
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
#[derive(Copy)]
struct XCBScreenIteratorFFI {
    data: *mut XCBScreenFFI,
    rem: c_int,
    index: c_int,
}

#[repr(C)]
#[derive(Copy)]
struct XCBUnnamed32FFI {
    sequence: c_uint,
}
impl Default for XCBUnnamed32FFI {
    fn default() -> XCBUnnamed32FFI { unsafe { mem::zeroed() } }
}
type XCBVoidCookieFFI = XCBUnnamed32FFI;


// FUNCTIONS

#[link(name = "xcb")]
extern {
    fn xcb_connect(displayname: *const c_char, screenp:*mut c_int) -> *mut XCBConnectionFFI;

    fn xcb_connection_has_error(c: *mut XCBConnectionFFI) -> c_int;

    fn xcb_disconnect(c: *mut XCBConnectionFFI);

    fn xcb_get_setup(c: *mut XCBConnectionFFI) -> *const XCBSetupFFI;

    fn xcb_setup_roots_iterator(R: *const XCBSetupFFI) -> XCBScreenIteratorFFI;

    fn xcb_generate_id(c: *mut XCBConnectionFFI) -> c_uint;

    fn xcb_create_gc(c: *mut XCBConnectionFFI, cid: XCBGcontextFFI, drawable: XCBDrawableFFI,
        value_mask: c_uint, value_list: *const c_uint) -> XCBVoidCookieFFI;
}


// PUB API

pub struct XCB {
    connection: *mut XCBConnectionFFI,

    screen: *mut XCBScreenFFI,

    //w: *mut XCBWindowFFI,
}


impl XCB {
    #[inline]
    pub fn new() -> Self {
        let mut screen: c_int = 0;
        unsafe {
            let connection = xcb_connect(ptr::null(), &mut screen);
            if xcb_connection_has_error(connection) > 0 {
                panic!("A XCB connection was not established due to a fatal error.")
            }

            let screen = screen_from_connection(connection);

            XCB {
                connection: connection,
                screen: screen,
            }
        }
    }

    #[inline]
    pub fn disconnect(&self) {
        unsafe {
            xcb_disconnect(self.connection);
        }
    }

    pub fn print_dimensions(&self) {
        unsafe {
            println!(
                "w: {}, h: {}",
                (*self.screen).width_in_pixels,
                (*self.screen).height_in_pixels
            );
        }
    }
}


#[inline]
fn screen_from_connection(connection: *mut XCBConnectionFFI) -> *mut XCBScreenFFI {
    unsafe {
        let iterator = xcb_setup_roots_iterator(
            xcb_get_setup(connection)
        );

        iterator.data
    }
}

