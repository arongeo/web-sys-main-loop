[![Crates.io Version](https://img.shields.io/crates/v/web-sys-main-loop)](https://crates.io/crates/web-sys-main-loop)
![Crates.io License](https://img.shields.io/crates/l/web-sys-main-loop)
[![docs.rs](https://img.shields.io/docsrs/web-sys-main-loop)](https://docs.rs/web-sys-main-loop/latest/web_sys_main_loop/)

# web-sys-main-loop

web-sys-main-loop as per the name suggests provides a main loop (in game development communities often called a game loop), for web-sys based WASM pages, with also providing input handling.

## [Documentation](https://docs.rs/web-sys-main-loop/latest/web_sys_main_loop/)
## Example
```rust
use web_sys_main_loop::FrameState;
...
let window = web_sys::window().unwrap();
...
web_sys_main_loop::start(&window, move |frame_state: FrameState| {
    ...
    // Gets the position (X, Y) of the cursor in the window
    // context
    let curr_position = frame_state.mouse_state.get_position();
    ...
});
```
