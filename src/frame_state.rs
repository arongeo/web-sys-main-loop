use crate::input_handler::mouse_state::MouseState;

/// Contains the input state for the current
/// frame.
///
/// pressed_keys provides the keycodes
/// the same way as
/// [MDN Web Docs' KeyboardEvent.code](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)
/// does
pub struct FrameState {
    pub pressed_keys: Vec<String>,
    pub mouse_state: MouseState,
}
