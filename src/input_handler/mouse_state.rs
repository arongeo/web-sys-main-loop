pub enum MouseButton {
    None,
    Left,
    Right,
    Scroll,
    Misc1,
    Misc2,
}

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

    pub fn get_position(&self) -> (i32, i32) {
        self.position
    }

    pub fn get_movement(&self) -> (i32, i32) {
        self.movement
    }

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
