// mode/menu_mode.rs
//
//

//use gilrs::EventType;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;

pub struct UIButton {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    name: String,
    next_mode: ModeTag
}

impl UIButton {
    fn new(left: i32, top: i32, right: i32, bottom: i32,
	   name: String, next_mode: ModeTag) -> UIButton {
	UIButton {
	    left: left,
	    top: top,
	    right: right,
	    bottom: bottom,
	    name: name,
	    next_mode: next_mode
	}
    }
}

pub struct MenuMode {
    screen : Texture2D,
    buttons: Vec<UIButton>,
    cursor_color: Color,
}

impl MenuMode {
    pub fn new(scr: Texture2D) -> MenuMode {
	let left = 800;
	let right = 1100;
	let button_top = 200;
	let button_bottom = 600;
	let button_margin = 10;
	let button_count = 6;
	let button_stack_height = button_bottom - button_top;
	let margin_space = (button_count - 1) * button_margin;
	let button_space = button_stack_height - margin_space;
	let button_height = button_space / button_count;
	
	MenuMode {
	    screen: scr,
	    buttons: vec!{
		UIButton::new(left,
			      button_top + 0 * (button_height + button_margin),
			      right,
			      button_top + button_height + 0 * (button_height + button_margin),
			      "play".to_string(),
			      ModeTag::PlayMode),
		UIButton::new(left,
			      button_top + 1 * (button_height + button_margin),
			      right,
			      button_top + button_height + 1 * (button_height + button_margin),
			      "new".to_string(),
			      ModeTag::NewGameMode),
		UIButton::new(left,
			      button_top + 2 * (button_height + button_margin),
			      right,
			      button_top + button_height + 2 * (button_height + button_margin),
			      "settings".to_string(),
			      ModeTag::SettingsMode),
		UIButton::new(left,
			      button_top + 3 * (button_height + button_margin),
			      right,
			      button_top + button_height + 3 * (button_height + button_margin),
			      "about".to_string(),
			      ModeTag::AboutMode),
		UIButton::new(left,
			      button_top + 4 * (button_height + button_margin),
			      right,
			      button_top + button_height + 4 * (button_height + button_margin),
			      "credits".to_string(),
			      ModeTag::CreditsMode),
		UIButton::new(left,
			      button_top + 5 * (button_height + button_margin),
			      right,
			      button_top + button_height + 5 * (button_height + button_margin),
			      "quit".to_string(),
			      ModeTag::QuitMode),
	    },
	    cursor_color: RED,
	}
    }

    fn click_in_button(&self, button: &UIButton, sx: i32, sy: i32) -> bool {
	(sx > button.left) && (sx <= button.right) &&
	    (sy >= button.top) && (sy <= button.bottom)
    }
}

impl GameMode for MenuMode {
    fn get_name(&self) -> String {
	"MenuMode".to_string()
    }

    fn init(&mut self) {
    }

    fn update(&mut self,
	      _dt_seconds: f32,
	      //_events: Vec<EventType>,
	      canvas: &Canvas2D) -> Option<ModeTag> {

	let (canvas_mouse_x, canvas_mouse_y) = canvas.mouse_position();

	self.cursor_color = RED;
	for btn in &self.buttons {
	    if self.click_in_button(btn,
				    canvas_mouse_x as i32,
				    canvas_mouse_y as i32) {
		if macroquad::input::is_mouse_button_pressed(MouseButton::Left) {
		    self.cursor_color = WHITE;
		    return Some(btn.next_mode);
		} else {
		    self.cursor_color = BLUE;
		}
	    }
	}
	
	None
    }

    fn draw(&self, _canvas:&Canvas2D) {
	//let (canvas_mouse_x, canvas_mouse_y) = canvas.mouse_position();
	
	draw_texture(self.screen,
		     0.0,
		     0.0,
		     WHITE);

	for btn in &self.buttons {
	    draw_rectangle(btn.left as f32, btn.top as f32,
			   (btn.right - btn.left) as f32,
			   (btn.bottom - btn.top) as f32,
			   DARKGRAY);

	    let font_size = 40.0;
	    
	    let text_dims = measure_text(&btn.name,
					 None,
					 font_size as u16,
					 1.0);

	    let btn_width = btn.right - btn.left;
	    let text_x_offset = (btn_width as f32 - text_dims.width) / 2.0;
	    let text_y_offset = 37.0; //btn_height - (btn_height - font_size) / 2.0;

	    

	    draw_text(&btn.name,
		      btn.left as f32 + text_x_offset,
		      btn.top as f32 + text_y_offset,
		      font_size,
		      WHITE);	    
	}

	/*
	let mut cursor_color = self.cursor_color;

	if macroquad::input::is_mouse_button_down(MouseButton::Left) {
	    cursor_color = GREEN;
	}*/

        //draw_circle(canvas_mouse_x, canvas_mouse_y, 20.0, cursor_color);
    }
}
