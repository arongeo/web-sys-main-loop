/// The possible buttons that can be pressed on a mouse.
pub enum MouseButton {
    None,
    Left,
    Right,
    Scroll,
    Misc1,
    Misc2,
}

/// Contains the state of the mouse
/// in the current frame
#[derive(Copy, Clone)]
pub struct MouseState {
    pub(crate) buttons_pressed: u8,
    pub(crate) position: (i32, i32),
    pub(crate) movement: (i32, i32),
    pub(crate) offset: (i32, i32),
}

impl MouseState {
    pub(crate) fn new() -> Self {
        Self {
            buttons_pressed: 0,
            position: (0, 0),
            movement: (0, 0),
            offset: (0, 0),
        }
    }

    /// Gets the currently pressed buttons on the mouse
    pub fn get_pressed_buttons(&self) -> Option<Vec<MouseButton>> {
        if self.buttons_pressed == 0 {
            return None;
        }

        let mut iter_num = 0;
        let mut buttons_pressed = self.buttons_pressed;
        let mut buttons = Vec::new();

        while iter_num < 5 {
            if (buttons_pressed & (1 << iter_num)) == 1 {
                buttons.push(match iter_num {
                    0 => MouseButton::Left,
                    1 => MouseButton::Right,
                    2 => MouseButton::Scroll,
                    3 => MouseButton::Misc1,
                    4 => MouseButton::Misc2,
                    _ => MouseButton::None,
                });
            }

            buttons_pressed /= 2;
            iter_num += 1;
        }

        return Some(buttons);
    }

    /// Gets the position (X, Y) of the cursor in the web_sys window
    ///
    /// [MDN Web Docs clientX property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    ///
    /// [MDN Web Docs clientY property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)
    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    /// Gets the movement (X, Y) of the cursor between the
    /// last mouse event and the current one.
    ///
    /// [MDN Web Docs movementX property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    ///
    /// [MDN Web Docs movementY property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    pub fn get_movement(&self) -> (i32, i32) {
        self.movement
    }

    /// Gets the offset (X, Y) between of the cursor between the
    /// event which caused it and the padding edge of the
    /// target node.
    ///
    /// [MDN Web Docs offsetX property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)
    ///
    /// [MDN Web Docs offsetY property](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)
    pub fn get_offset(&self) -> (i32, i32) {
        self.offset
    }

    pub(crate) fn copy(&self) -> Self {
        Self {
            buttons_pressed: self.buttons_pressed,
            position: self.position,
            offset: self.offset,
            movement: self.movement,
        }
    }
}
