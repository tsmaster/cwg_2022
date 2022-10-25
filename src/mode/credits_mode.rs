// mode/credits_mode.rs
//
//

use gilrs::EventType;
use macroquad::prelude::*;

use crate::mode::GameMode;
use crate::mode::ModeTag;

pub struct CreditsMode {
    screen : Texture2D,
}

impl CreditsMode {
    pub fn new(scr: Texture2D) -> CreditsMode {
	CreditsMode {
	    screen: scr,
	}
    }
}

impl GameMode for CreditsMode {
    fn get_name(&self) -> String {
	"CreditsMode".to_string()
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
