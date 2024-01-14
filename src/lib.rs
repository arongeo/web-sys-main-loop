// Copyright 2024 arongeo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! web-sys-main-loop provides a main loop (or game loop),
//! and input handling, for a web_sys window.
//!
//! Example:
//! ```
//! use web_sys_main_loop::FrameState;
//!
//! let window = web_sys::window().unwrap();
//!
//! web_sys_main_loop::start(&window, move |frame_state: FrameState| {
//!     ...
//!     // Gets the position (X, Y) of the cursor in the window
//!     // context
//!     let curr_position = frame_state.mouse_state.get_position();
//!     ...
//! });
//!
//! ```

mod frame_state;
mod input_handler;
mod main_loop;

pub use crate::frame_state::FrameState;
pub use crate::input_handler::mouse_state::MouseButton;
pub use crate::input_handler::mouse_state::MouseState;
pub use crate::main_loop::start;
