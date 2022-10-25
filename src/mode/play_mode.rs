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
    elapsed_seconds : f32,
    display_seconds : f32,
    screen : Texture2D,
}

impl PlayMode {
    pub fn new(scr: Texture2D) -> PlayMode {
	PlayMode {
	    elapsed_seconds: 0.0,
	    display_seconds: 5.0,
	    screen: scr,
	}
    }
}

impl GameMode for PlayMode {
    fn get_name(&self) -> String {
	"PlayMode".to_string()
    }

    fn init(&mut self) {
	self.elapsed_seconds = 0.0
    }

    fn update(&mut self,
	      dt_seconds: f32,
	      //events: Vec<EventType>,
	      _canvas: &Canvas2D) -> Option<ModeTag> {
	self.elapsed_seconds += dt_seconds;

	if self.elapsed_seconds > self.display_seconds {
	    return Some(ModeTag::CwgTitleMode);
	}

	/*
	for e in events {
	    match e {
		ButtonPressed(dir, _code) => {
		    println!("play button {:?}", dir);
		    return Some(ModeTag::CwgTitleMode);
		},
		_ => {}
	    }
	}*/
	
	None
    }

    fn draw(&self, _canvas:&Canvas2D) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);
    }
}
