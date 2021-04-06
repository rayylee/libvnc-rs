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
//!     rfb_attach_framebuffer(server, 400*300*4);
//!     rfb_init_server(server);
//!     rfb_run_event_loop(server);
//! }
//! ```


use std;

include!(concat!(std::env!("OUT_DIR"), "/rfb.rs"));

pub type RfbScreenInfoPtr = rfbScreenInfoPtr;

pub fn rfb_get_screen(width: i32, height: i32, bits_per_sample: i32, samples_per_pixel: i32, bytes_per_pixel: i32) -> RfbScreenInfoPtr {
    let mut arg_len = 0 as i32;
    let mut arg_ptr: *mut i8 = std::ptr::null_mut();

    unsafe {
        let server = rfbGetScreen(&mut arg_len, &mut arg_ptr, width, height, bits_per_sample, samples_per_pixel, bytes_per_pixel);
        server
    }
}

pub fn rfb_attach_framebuffer(ptr: RfbScreenInfoPtr, size: u64) {
    unsafe {
        (*ptr).frameBuffer = malloc(size as ::std::os::raw::c_ulong) as *mut i8;
    }
}

pub fn rfb_init_server(ptr: RfbScreenInfoPtr) {
    unsafe {
        rfbInitServerWithPthreadsAndZRLE(ptr);
    }
}

pub fn rfb_run_event_loop(ptr: RfbScreenInfoPtr) {
    unsafe {
        rfbRunEventLoop(ptr, -1, 0);
    };
}
