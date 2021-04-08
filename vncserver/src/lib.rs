mod rfb;

pub type RfbScreenInfoPtr = rfb::rfbScreenInfoPtr;
pub type RfbBool = rfb::rfbBool;

pub fn rfb_get_screen(width: i32, height: i32, bits_per_sample: i32, samples_per_pixel: i32, bytes_per_pixel: i32) -> RfbScreenInfoPtr {
    rfb::rfb_get_screen(width, height, bits_per_sample, samples_per_pixel, bytes_per_pixel)
}

pub fn rfb_screen_cleanup(ptr: RfbScreenInfoPtr) {
    rfb::rfb_screen_cleanup(ptr);
}

pub fn rfb_framebuffer_malloc(ptr: RfbScreenInfoPtr, fb_size: u64) {
    rfb::rfb_framebuffer_malloc(ptr, fb_size);
}

pub fn rfb_framebuffer_free(ptr: RfbScreenInfoPtr) {
    rfb::rfb_framebuffer_free(ptr);
}

pub fn rfb_framebuffer_set_rgb16(ptr: RfbScreenInfoPtr, x: i32, y: i32, rgb16: u16) {
    rfb::rfb_framebuffer_set_rgb16(ptr, x, y, rgb16);
}

pub fn rfb_init_server(ptr: RfbScreenInfoPtr) {
    rfb::rfb_init_server(ptr);
}

pub fn rfb_run_event_loop(ptr: RfbScreenInfoPtr, usec: i64, run_in_background: RfbBool) {
    rfb::rfb_run_event_loop(ptr, usec, run_in_background);
}
