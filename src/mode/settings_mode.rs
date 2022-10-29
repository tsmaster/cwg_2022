// mode/settings_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::ButtonChanged;
//use gilrs::EventType::ButtonPressed;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;

pub struct SettingsMode {
    screen : Texture2D,
}

impl SettingsMode {
    pub fn new(scr: Texture2D) -> SettingsMode {
	SettingsMode {
	    screen: scr,
	}
    }
}

impl GameMode for SettingsMode {
    fn get_name(&self) -> String {
	"SettingsMode".to_string()
    }

    fn init(&mut self) {
    }

    fn update(&mut self,
	      _dt_seconds: f32,
	      _canvas: &Canvas2D) -> Option<ModeTag> {
	if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
	    return Some(ModeTag::MenuMode);
	}

	if is_key_pressed(KeyCode::Escape) {
	    return Some(ModeTag::MenuMode);
	}
	
	None
    }

    fn draw(&self, _canvas:&Canvas2D) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);
    }
}
