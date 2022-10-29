// mode/play_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::ButtonChanged;
//use gilrs::EventType::ButtonPressed;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;

pub struct PlayMode {
    screen : Texture2D,
}

impl PlayMode {
    pub fn new(scr: Texture2D) -> PlayMode {
	PlayMode {
	    screen: scr,
	}
    }
}

impl GameMode for PlayMode {
    fn get_name(&self) -> String {
	"PlayMode".to_string()
    }

    fn init(&mut self) {
    }

    fn update(&mut self,
	      _dt_seconds: f32,
	      //events: Vec<EventType>,
	      _canvas: &Canvas2D) -> Option<ModeTag> {

	Some(ModeTag::ArenaMode)
    }

    fn draw(&self, _canvas:&Canvas2D) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);
    }
}
