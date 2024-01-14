use crate::input_handler::mouse_state::MouseState;

pub struct FrameState {
    pub(crate) pressed_keys: Vec<String>,
    pub(crate) mouse_state: MouseState,
}
