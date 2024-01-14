use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn keydown_setup(window: &web_sys::Window, pressed_keys: Rc<RefCell<Vec<String>>>) {
    let keydown_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut kdh_pressed_keys = pressed_keys.borrow_mut();
        if !kdh_pressed_keys.contains(&event.code()) {
            kdh_pressed_keys.push(event.code());
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref())
        .unwrap();

    keydown_handler.forget();
}

fn keyup_setup(window: &web_sys::Window, pressed_keys: Rc<RefCell<Vec<String>>>) {
    let keyup_handler = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut kuh_pressed_keys = pressed_keys.borrow_mut();
        kuh_pressed_keys.retain(|key| *key != event.code());
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keyup", keyup_handler.as_ref().unchecked_ref())
        .unwrap();

    keyup_handler.forget();
}

pub(crate) fn input_handler_setup(
    window: &web_sys::Window,
    pressed_keys: Rc<RefCell<Vec<String>>>,
) {
    // https://stackoverflow.com/questions/53214434/how-to-return-a-rust-closure-to-javascript-via-webassembly/53219594#53219594
    keydown_setup(window, pressed_keys.clone());
    keyup_setup(window, pressed_keys.clone());
}
