mod frame_state;
mod input_handler;
mod main_loop;

pub mod prelude {
    pub use crate::input_handler::mouse_state::MouseState;
    pub use crate::main_loop::start;
}
