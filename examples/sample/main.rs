use vncserver::*;

fn main() {
    println!("Hello, world!");

    let server = rfb_get_screen(400, 300, 8, 3, 4);
    rfb_framebuffer_malloc(server, 400*300*4);
    rfb_init_server(server);
    rfb_run_event_loop(server, -1, 0);
}

