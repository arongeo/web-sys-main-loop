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

use crate::frame_state::FrameState;
use crate::input_handler::keyboard_state;
use crate::input_handler::mouse_state::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn mouse_state_parser_setup(window: &web_sys::Window, mouse_state: Rc<RefCell<MouseState>>) {
    let mouse_event_handler = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let mut mouse_state = mouse_state.borrow_mut();

        mouse_state.position = (event.x(), event.y());
        mouse_state.offset = (event.offset_x(), event.offset_y());
        mouse_state.movement = (event.movement_x(), event.movement_y());
        mouse_state.buttons_pressed = event.buttons() as u8;
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("mousedown", mouse_event_handler.as_ref().unchecked_ref())
        .unwrap();

    window
        .add_event_listener_with_callback("mouseup", mouse_event_handler.as_ref().unchecked_ref())
        .unwrap();

    window
        .add_event_listener_with_callback("mousemove", mouse_event_handler.as_ref().unchecked_ref())
        .unwrap();

    mouse_event_handler.forget();
}

pub(crate) struct InputState {
    pub(crate) mouse_state: Rc<RefCell<MouseState>>,
    pub(crate) pressed_keys: Rc<RefCell<Vec<String>>>,
}

impl InputState {
    pub(crate) fn new(window: &web_sys::Window) -> Self {
        let mouse_state = Rc::new(RefCell::new(MouseState::new()));
        let pressed_keys = Rc::new(RefCell::new(Vec::new()));

        mouse_state_parser_setup(window, mouse_state.clone());
        keyboard_state::input_handler_setup(window, pressed_keys.clone());

        Self {
            mouse_state,
            pressed_keys,
        }
    }

    pub(crate) fn clear(&self) {
        let mut mouse_state = self.mouse_state.borrow_mut();

        mouse_state.position = (0, 0);
        mouse_state.movement = (0, 0);
        mouse_state.offset = (0, 0);
        mouse_state.buttons_pressed = 0;
    }

    pub(crate) fn create_frame_state(&self) -> FrameState {
        FrameState {
            mouse_state: self.mouse_state.borrow().copy(),
            pressed_keys: self.pressed_keys.borrow().to_vec(),
        }
    }
}
