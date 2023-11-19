use vncserver::*;

fn main() {
    let server = rfb_get_screen(400, 300, 5, 3, 2);
    rfb_framebuffer_malloc(server, 400 * 300 * 2);

    let mut j = 0;
    while j < 100 {
        for i in 0..400 {
            rfb_framebuffer_set_rgb16(server, i, j, 0xF800);
        }
        j += 1;
    }
    while j < 200 {
        for i in 0..400 {
            rfb_framebuffer_set_rgb16(server, i, j, 0xFFFF);
        }
        j += 1;
    }
    while j < 300 {
        for i in 0..400 {
            rfb_framebuffer_set_rgb16(server, i, j, 0x0000);
        }
        j += 1;
    }

    rfb_init_server(server);
    rfb_run_event_loop(server, -1, 0);

    rfb_framebuffer_free(server);
    rfb_screen_cleanup(server);
}
