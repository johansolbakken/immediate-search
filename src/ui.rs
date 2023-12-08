use std::collections::HashMap;

use raylib::{input::key_from_i32, prelude::*};

pub struct UI {
    active: usize,
    text_inputs: HashMap<usize, TextInput>,
}

struct TextInput {
    text: String,
    cursor: usize,
}

impl UI {
    pub fn new() -> Self {
        Self {
            active: 0,
            text_inputs: HashMap::new(),
        }
    }

    pub fn button(
        &mut self,
        d: &mut RaylibDrawHandle,
        id: usize,
        text: &str,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> bool {
        let active = id == self.active;
        let mouse = d.get_mouse_position();
        let mouse_over = mouse.x >= x as f32
            && mouse.x <= (x + w) as f32
            && mouse.y >= y as f32
            && mouse.y <= (y + h) as f32;
        let color = if active {
            Color::new(158, 200, 185, 255)
        } else if mouse_over {
            Color::new(92, 131, 116, 255)
        } else {
            Color::new(27, 66, 66, 255)
        };
        d.draw_rectangle(x, y, w, h, color);
        let center_y = y + h / 2 - 10;
        d.draw_text(text, x + 8, center_y, 20, Color::WHITE);

        if mouse_over && d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            self.active = id;
            return false;
        }

        if active && d.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON) {
            self.active = 0;
            if mouse_over {
                return true;
            }
        }
        false
    }

    /*
    let mut search = search_root.clone();
        if ui.text_input(&mut d, 6, &mut search, x, y, 200, 50) {
            search_root = search;
        }
     */

    pub fn text_input(
        &mut self,
        d: &mut RaylibDrawHandle,
        id: usize,
        text: &mut String,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    ) -> bool {
        if !self.text_inputs.contains_key(&id) {
            self.text_inputs.insert(
                id,
                TextInput {
                    text: text.clone(),
                    cursor: 0,
                },
            );
        }

        let text_input = self.text_inputs.get_mut(&id).unwrap();
        let active = id == self.active;
        let mouse = d.get_mouse_position();
        let mouse_over = mouse.x >= x as f32
            && mouse.x <= (x + w) as f32
            && mouse.y >= y as f32
            && mouse.y <= (y + h) as f32;
        let color = if active {
            Color::new(158, 200, 185, 255)
        } else if mouse_over {
            Color::new(92, 131, 116, 255)
        } else {
            Color::new(27, 66, 66, 255)
        };
        d.draw_rectangle(x, y, w, h, color);
        let center_y = y + h / 2 - 10;

        let mut text_to_draw = text_input.text.clone();
        text_to_draw.insert(text_input.cursor, '|');
        d.draw_text(text_to_draw.as_str(), x + 8 , center_y, 20, Color::WHITE);

        if mouse_over && d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            self.active = id;
            return false;
        }

        if active {
            if d.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
                if text_input.cursor > 0 {
                    text_input.text.remove(text_input.cursor - 1);
                    text_input.cursor -= 1;
                }
            } else if d.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                if text_input.cursor < text_input.text.len() {
                    text_input.cursor += 1;
                }
            } else if d.is_key_pressed(KeyboardKey::KEY_LEFT) {
                if text_input.cursor > 0 {
                    text_input.cursor -= 1;
                }
            } else if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.active = 0;
                *text = text_input.text.clone();
                return true;
            } else if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                self.active = 0;
            } else {
                let key = unsafe { ffi::GetKeyPressed() };
                if key > 0 {
                    if let Some(key) = key_from_i32(key) {
                        let c = key as u8;
                        text_input.text.insert(text_input.cursor, c as char);
                        text_input.cursor += 1;
                    }
                }
            }
        }

        false
    }
}
