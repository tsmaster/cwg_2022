// mode/menu_mode.rs
//
//

use gilrs::EventType;
use macroquad::prelude::*;

use crate::mode::GameMode;
use crate::mode::ModeTag;

pub struct MenuMode {
    screen : Texture2D,
}

impl MenuMode {
    pub fn new(scr: Texture2D) -> MenuMode {
	MenuMode {
	    screen: scr,
	}
    }
}

impl GameMode for MenuMode {
    fn get_name(&self) -> String {
	"MenuMode".to_string()
    }

    fn init(&mut self) {
    }

    fn update(&mut self, dt_seconds: f32, events: Vec<EventType>) -> Option<ModeTag> {

	
	None
    }

    fn draw(&self) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);
    }
}
