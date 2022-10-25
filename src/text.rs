// src/text.rs
//
// functions for working with text, including drawing text on the
// canvas

use macroquad::prelude::*;

pub fn draw_outlined(msg: &str,
		     x: f32, y: f32,
		     size: f32,
		     fill_color: Color, outline_color: Color) {
    for dx in -2 ..= 2 {
	for dy in -2 ..= 2 {
	    draw_text(msg, x + dx as f32,
		      y + dy as f32,
		      size,
		      outline_color);
	}
    }
    draw_text(msg, x, y, size, fill_color);
}

