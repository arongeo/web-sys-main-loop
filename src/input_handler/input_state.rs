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

    window.add_event_listener_with_callback(
        "mousedown",
        mouse_event_handler.as_ref().unchecked_ref(),
    );

    window
        .add_event_listener_with_callback("mouseup", mouse_event_handler.as_ref().unchecked_ref());

    window.add_event_listener_with_callback(
        "mousemove",
        mouse_event_handler.as_ref().unchecked_ref(),
    );
}

pub struct InputState {
    mouse_state: Rc<RefCell<MouseState>>,
}

impl InputState {
    pub(crate) fn new(window: &web_sys::Window) -> Self {
        let mut mouse_state = Rc::new(RefCell::new(MouseState::new()));

        mouse_state_parser_setup(window, mouse_state.clone());

        Self { mouse_state }
    }

    pub(crate) fn clear(&self) {
        let mut mouse_state = self.mouse_state.borrow_mut();

        mouse_state.position = (0, 0);
        mouse_state.movement = (0, 0);
        mouse_state.offset = (0, 0);
        mouse_state.buttons_pressed = 0;
    }
}
