#![allow(improper_ctypes)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

//! # Libvncserver bindings
//!
//! This library contains safe Rust bindings for [Libvncserver](https://github.com/LibVNC/libvncserver)
//!
//! # Sample
//!
//! ```no_run
//! use vncserver::*;
//! fn main() {
//!     let server = rfb_get_screen(400, 300, 8, 3, 4);
//!     rfb_framebuffer_malloc(server, 400*300*4);
//!     rfb_init_server(server);
//!     rfb_run_event_loop(server, -1, 0);
//! }
//! ```

use std;
use core::slice;

include!(concat!(std::env!("OUT_DIR"), "/rfb.rs"));


pub type RfbScreenInfoPtr = rfbScreenInfoPtr;
pub type RfbBool = rfbBool;

pub fn rfb_get_screen(width: i32, height: i32, bits_per_sample: i32, samples_per_pixel: i32, bytes_per_pixel: i32) -> RfbScreenInfoPtr {
    let mut arg_len = 0 as i32;
    let mut arg_ptr: *mut i8 = std::ptr::null_mut();

    unsafe {
        let server = rfbGetScreen(&mut arg_len, &mut arg_ptr, width, height, bits_per_sample, samples_per_pixel, bytes_per_pixel);
        server
    }
}

pub fn rfb_screen_cleanup(ptr: RfbScreenInfoPtr) {
    unsafe {
        rfbScreenCleanup(ptr);
    }
}

pub fn rfb_framebuffer_malloc(ptr: RfbScreenInfoPtr, fb_size: u64) {
    unsafe {
        (*ptr).frameBuffer = malloc(fb_size as ::std::os::raw::c_ulong) as *mut i8;
    }
}

pub fn rfb_framebuffer_free(ptr: RfbScreenInfoPtr) {
    unsafe {
        free((*ptr).frameBuffer as *mut ::std::os::raw::c_void);
    }
}

pub fn rfb_framebuffer_set_rgb16(ptr: RfbScreenInfoPtr, x: i32, y: i32, rgb16: u16) {
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

pub fn rfb_init_server(ptr: RfbScreenInfoPtr) {
    unsafe {
        rfbInitServerWithPthreadsAndZRLE(ptr);
    }
}

pub fn rfb_run_event_loop(ptr: RfbScreenInfoPtr, usec: i64, run_in_background: RfbBool) {
    unsafe {
        rfbRunEventLoop(ptr, usec as ::std::os::raw::c_long, run_in_background);
    };
}
