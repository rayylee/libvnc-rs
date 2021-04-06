# libvnc-rs

The `libvnc-rs` repo aims to provide safe Rust binding over `Libvncserver`-based libraries.
You can find more about `Libvncserver` on <https://github.com/LibVNC/libvncserver>.


## Example

```
use vncserver::*;
fn main() {
    let server = rfb_get_screen(400, 300, 8, 3, 4);
    rfb_attach_framebuffer(server, 400*300*4);
    rfb_init_server(server);
    rfb_run_event_loop(server);
}
```


## Getting Started

You can quickly try out the [examples](examples) by cloning this repo and running the following commands:

```sh
# Runs the "sample" example
cargo run --example sample
```


## WARNING

The `libvnc-rs` is still in the very early stages of development. APIs can and will change. Important features are missing.
