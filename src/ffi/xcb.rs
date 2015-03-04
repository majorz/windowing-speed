use libc::{c_int, c_char, c_uchar, c_ushort, c_uint};
use std::ptr;
use std::mem;
use std::default::Default;

// CONSTANTS

const XCB_WINDOW_CLASS_INPUT_OUTPUT: c_ushort = 1;

const XCB_GC_FOREGROUND: c_uint = 4;
const XCB_GC_GRAPHICS_EXPOSURES: c_uint = 65536;

const XCB_CW_BACK_PIXEL: c_uint = 2;
const XCB_CW_EVENT_MASK: c_uint = 2048;

const XCB_EVENT_MASK_KEY_PRESS: c_uint = 1;
const XCB_EVENT_MASK_EXPOSURE: c_uint = 32768;


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
struct XCBVoidCookieFFI {
    sequence: c_uint,
}

impl Default for XCBVoidCookieFFI {
    fn default() -> XCBVoidCookieFFI { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
struct XCBGenericEventFFI {
    response_type: c_uchar,
    pad0: c_uchar,
    sequence: c_ushort,
    pad: [c_uint; 7usize],
    full_sequence: c_uint,
}

impl Default for XCBGenericEventFFI {
    fn default() -> XCBGenericEventFFI { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct XCBRectangleFFI {
    pub x: c_ushort,
    pub y: c_ushort,
    pub width: c_ushort,
    pub height: c_ushort,
}

impl Default for XCBRectangleFFI {
    fn default() -> XCBRectangleFFI { unsafe { mem::zeroed() } }
}


// FUNCTIONS

#[link(name = "xcb")]
extern {
    fn xcb_connect(displayname: *const c_char, screenp:*mut c_int) -> *mut XCBConnectionFFI;

    fn xcb_connection_has_error(c: *mut XCBConnectionFFI) -> c_int;

    fn xcb_disconnect(c: *mut XCBConnectionFFI);

    fn xcb_get_setup(c: *mut XCBConnectionFFI) -> *const XCBSetupFFI;

    fn xcb_setup_roots_iterator(R: *const XCBSetupFFI) -> XCBScreenIteratorFFI;

    fn xcb_generate_id(c: *mut XCBConnectionFFI) -> c_uint;

    fn xcb_create_gc(
        c: *mut XCBConnectionFFI,
        cid: XCBGcontextFFI,
        drawable: XCBDrawableFFI,
        value_mask: c_uint,
        value_list: *const c_uint
    ) -> XCBVoidCookieFFI;

    fn xcb_create_window(
        c: *mut XCBConnectionFFI,
        depth: c_uchar,
        wid: XCBWindowFFI,
        parent: XCBWindowFFI,
        x: c_ushort,
        y: c_ushort,
        width: c_ushort,
        height: c_ushort,
        border_width: c_ushort,
        _class: c_ushort,
        visual: XCBVisualidFFI,
        value_mask: c_uint,
        value_list: *const c_uint
    ) -> XCBVoidCookieFFI;

    fn xcb_map_window(c: *mut XCBConnectionFFI, window: XCBWindowFFI) -> XCBVoidCookieFFI;

    fn xcb_flush(c: *mut XCBConnectionFFI) -> c_int;

    fn xcb_wait_for_event(c: *mut XCBConnectionFFI) -> *mut XCBGenericEventFFI;

    fn xcb_poly_rectangle(
        c: *mut XCBConnectionFFI,
        drawable: XCBDrawableFFI,
        gc: XCBGcontextFFI,
        rectangles_len: c_uint,
        rectangles: *const XCBRectangleFFI
    ) -> XCBVoidCookieFFI;

}


// PUB API

pub struct XCB {
    connection: *mut XCBConnectionFFI,

    screen: Option<*mut XCBScreenFFI>,

    window: Option<XCBWindowFFI>,
}


impl XCB {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut screen: c_int = 0;
            let connection = xcb_connect(ptr::null(), &mut screen);

            if xcb_connection_has_error(connection) > 0 {
                panic!("Fatal error during establishing a XCB connection.")
            }

            XCB {
                connection: connection,
                screen: None,
                window: None,
            }
        }
    }

    #[inline]
    pub fn disconnect(&self) {
        unsafe {
            xcb_disconnect(self.connection);
        }
    }

    pub fn create_window(&mut self) {
        self.init_screen();

        unsafe {
            let window = xcb_generate_id(self.connection);
            self.window = Some(window);

            let screen = *self.screen.unwrap();

            let mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
            let value_list: [u32; 2] = [
                screen.white_pixel,
                XCB_EVENT_MASK_EXPOSURE | XCB_EVENT_MASK_KEY_PRESS
            ];

            xcb_create_window(
                self.connection,
                0, // Depth - copy from parent
                window,
                screen.root,
                0, 0,
                screen.width_in_pixels, screen.height_in_pixels,
                0,
                XCB_WINDOW_CLASS_INPUT_OUTPUT,
                screen.root_visual,
                mask, value_list.as_ptr(),
            );

            xcb_map_window(self.connection, window);

            xcb_flush(self.connection);
        }
    }

    pub fn exec(&self) {
        unsafe {
            let screen = *self.screen.unwrap();
            let windowp = self.window.unwrap();

            let foreground = xcb_generate_id(self.connection);
            let mask = XCB_GC_FOREGROUND | XCB_GC_GRAPHICS_EXPOSURES;
            let value_list: [u32; 2] = [screen.black_pixel, 0];
            xcb_create_gc(self.connection, foreground, windowp, mask, value_list.as_ptr());

            let rectangles: [XCBRectangleFFI; 2] = [
                XCBRectangleFFI {
                    x: 100,
                    y: 100,
                    width: 150,
                    height: 100,
                },
                XCBRectangleFFI {
                    x: 300,
                    y: 100,
                    width: 250,
                    height: 100,
                }
            ];

            loop {
                let event = xcb_wait_for_event(self.connection);
                let event_type = (*event).response_type & !0x80;

                if event_type == 12 { // XCB_EXPOSE
                    xcb_poly_rectangle(self.connection, windowp, foreground, 2, rectangles.as_ptr());
                    xcb_flush(self.connection);
                } else if event_type ==  2 { // XCB_KEY_PRESS
                    break;
                }
            }
        }
    }

    fn init_screen(&mut self) {
        match self.screen {
            None => unsafe {
                let iterator = xcb_setup_roots_iterator(
                    xcb_get_setup(self.connection)
                );

                self.screen = Some(iterator.data);
            },
            Some(_) => ()
        }
    }
}

