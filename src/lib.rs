mod frame_state;
mod input_handler;
mod main_loop;

pub use crate::frame_state::FrameState;
pub use crate::input_handler::mouse_state::MouseButton;
pub use crate::input_handler::mouse_state::MouseState;
pub use crate::main_loop::start;
