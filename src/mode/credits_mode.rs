// mode/credits_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::*;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;
use crate::text::*;

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

    fn update(&mut self,
	      _dt_seconds: f32,
	      //events: Vec<EventType>,
	      _canvas: &Canvas2D) -> Option<ModeTag> {

	/*
	for evt in events {
	    
	    match evt {
		ButtonPressed(_dir, _code) => {
		    return Some(ModeTag::MenuMode);
		},
		_ => {}
	    }
	}*/
	
	None
    }

    fn draw(&self, _canvas: &Canvas2D) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);

	let credits_text = "
Credits

Code:        Dave LeCompte
Some Assets: Kenney.nl

";
	for (i,line) in credits_text.lines().enumerate() {
	    draw_outlined(line,
			  50.0, 50.0 + i as f32 * 50.0, 50.0,
			  WHITE, DARKGRAY);
	}
    }
}
