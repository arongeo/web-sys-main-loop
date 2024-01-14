use crate::input_handler::mouse_state::MouseState;

pub struct FrameState {
    pub pressed_keys: Vec<String>,
    pub mouse_state: MouseState,
}
