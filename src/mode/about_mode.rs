// mode/about_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::ButtonChanged;
//use gilrs::EventType::ButtonPressed;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;
use crate::text::*;

pub struct AboutMode {
    screen : Texture2D,
}

impl AboutMode {
    pub fn new(scr: Texture2D) -> AboutMode {
	AboutMode {
	    screen: scr,
	}
    }
}

impl GameMode for AboutMode {
    fn get_name(&self) -> String {
	"AboutMode".to_string()
    }

    fn init(&mut self) {
    }

    fn update(&mut self,
	      _dt_seconds: f32,
	      //events: Vec<EventType>,
	      _canvas: &Canvas2D) -> Option<ModeTag> {

	/*
	for e in events {
	    match e {
		ButtonPressed(_dir, _code) => {
		    //println!("about button {:?}", dir);
		    return Some(ModeTag::MenuMode);
		},
		_ => {}
	    }
	}
	*/
	None
    }

    fn draw(&self, _canvas:&Canvas2D) {
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);

	let about_text = "
Cars With Guns (2022)

This is a game about cars. Maybe with guns.

Cars are not yet implemented. Guns are not yet 
implemented.

I'm writing this in Rust, as an effort to learn 
Rust.
";
	for (i,line) in about_text.lines().enumerate() {
	    draw_outlined(line,
			  50.0, 50.0 + i as f32 * 50.0, 50.0,
			  WHITE, DARKGRAY);
	    
	}
    }
}
