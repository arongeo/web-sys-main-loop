// Copyright 2024 arongeo
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::frame_state::FrameState;
use crate::input_handler::input_state::InputState;

fn request_animation_frame(window: &web_sys::Window, function: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(function.as_ref().unchecked_ref())
        .expect("Frame dropped");
}

/// Runs the closure provided with [requestAnimationFrame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)
/// every possible frame
///
/// Use it as:
/// ```
/// use web_sys_main_loop::{start, FrameState};
///
/// let window = web_sys::window().unwrap();
/// ...
/// start(&window, move |frame_state: FrameState| {
///     ...
/// });
///
/// ```
pub fn start<F>(window: &web_sys::Window, mut main_loop_function: F)
where
    F: FnMut(FrameState) + 'static,
{
    let main_loop = Rc::new(RefCell::new(None));
    let main_loop_starter = main_loop.clone();
    let window_clone = window.clone();

    let input_state = InputState::new(&window);

    *main_loop_starter.borrow_mut() = Some(Closure::new(move || {
        main_loop_function(input_state.create_frame_state());

        input_state.clear();

        request_animation_frame(&window_clone, main_loop.borrow().as_ref().unwrap());
    }));

    request_animation_frame(&window, main_loop_starter.borrow().as_ref().unwrap());
}
