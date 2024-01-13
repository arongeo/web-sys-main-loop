use crate::frame_state::FrameState;
use crate::input_handler::input_state::InputState;
use crate::input_handler::mouse_state::MouseButton;

pub struct MainLoop {
    window: web_sys::Window,
    input_state: InputState,
}

impl MainLoop {
    pub fn start<F>(window: &web_sys::Window, main_loop_function: F)
    where
        F: FnMut(FrameState),
    {
    }
}
