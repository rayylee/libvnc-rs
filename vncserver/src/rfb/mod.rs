#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std;
use core::slice;

include!(concat!(std::env!("OUT_DIR"), "/rfb.rs"));


pub fn rfb_get_screen(width: i32, height: i32, bits_per_sample: i32, samples_per_pixel: i32, bytes_per_pixel: i32) -> rfbScreenInfoPtr {
    let mut arg_len = 0 as i32;
    let mut arg_ptr: *mut i8 = std::ptr::null_mut();

    unsafe {
        let server = rfbGetScreen(&mut arg_len, &mut arg_ptr, width, height, bits_per_sample, samples_per_pixel, bytes_per_pixel);
        server
    }
}

pub fn rfb_screen_cleanup(ptr: rfbScreenInfoPtr) {
    unsafe {
        rfbScreenCleanup(ptr);
    }
}

pub fn rfb_framebuffer_malloc(ptr: rfbScreenInfoPtr, fb_size: u64) {
    unsafe {
        (*ptr).frameBuffer = malloc(fb_size as ::std::os::raw::c_ulong) as *mut i8;
    }
}

pub fn rfb_framebuffer_free(ptr: rfbScreenInfoPtr) {
    unsafe {
        free((*ptr).frameBuffer as *mut ::std::os::raw::c_void);
    }
}

pub fn rfb_framebuffer_set_rgb16(ptr: rfbScreenInfoPtr, x: i32, y: i32, rgb16: u16) {
	unsafe {
        let addr = (*ptr).frameBuffer as *mut u16;
        let fb_size = (*ptr).height * (*ptr).width * (*ptr).bitsPerPixel / 2;
        let slice: &mut [u16] = slice::from_raw_parts_mut(addr, fb_size as usize);
        let pos = (*ptr).width*y + x;
        if pos < fb_size {
            slice[pos as usize] = rgb16;
        }
	}
}

pub fn rfb_process_events(ptr: rfbScreenInfoPtr, usec: i64) -> rfbBool {
    unsafe {
        rfbProcessEvents(ptr, usec as ::std::os::raw::c_long)
    }
}

pub fn rfb_kbd_add_event(ptr: rfbScreenInfoPtr, cb: rfbKbdAddEventProcPtr) {
    unsafe {
        (*ptr).kbdAddEvent = cb;
    }
}

pub fn rfb_mark_rect_as_modified(ptr: rfbScreenInfoPtr, x1: i32, y1: i32, x2: i32, y2: i32) {
	unsafe {
        rfbMarkRectAsModified(ptr,
            x1 as ::std::os::raw::c_int,
            y1 as ::std::os::raw::c_int,
            x2 as ::std::os::raw::c_int,
            y2 as ::std::os::raw::c_int);
	}
}

pub fn rfb_is_active(ptr: rfbScreenInfoPtr) -> rfbBool {
    unsafe {
        rfbIsActive(ptr)
    }
}

pub fn rfb_init_server(ptr: rfbScreenInfoPtr) {
    unsafe {
        rfbInitServerWithPthreadsAndZRLE(ptr);
    }
}

pub fn rfb_run_event_loop(ptr: rfbScreenInfoPtr, usec: i64, run_in_background: rfbBool) {
    unsafe {
        rfbRunEventLoop(ptr, usec as ::std::os::raw::c_long, run_in_background);
    };
}
