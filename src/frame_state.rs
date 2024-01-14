use crate::input_handler::mouse_state::MouseState;

/// Contains the input state for the current
/// frame.
///
pub struct FrameState {
    /// Provides the keycodes the same way as
    /// [MDN Web Docs' KeyboardEvent.code](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code)
    /// does
    pub pressed_keys: Vec<String>,
    /// Contains the state of the mouse in the current frame
    pub mouse_state: MouseState,
}
