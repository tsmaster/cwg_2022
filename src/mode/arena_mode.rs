// mode/arena_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::ButtonChanged;
//use gilrs::EventType::ButtonPressed;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::GameMode;
use crate::mode::ModeTag;


fn copy_params(in_param: &DrawTextureParams) -> DrawTextureParams {
    DrawTextureParams {
	dest_size: in_param.dest_size,
	source: in_param.source,
	rotation: in_param.rotation,
	flip_x: in_param.flip_x,
	flip_y: in_param.flip_y,
	pivot: in_param.pivot
    }
}


fn make_params_vec() -> Vec<DrawTextureParams> {
    let mut params = vec!();

    // tree
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 4.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: false,
	pivot: None
    });

    // tree (flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 4.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: false,
	pivot: None
    });

    
    // trees 
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 5.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: false,
	pivot: None
    });

    // trees (flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 5.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: false,
	pivot: None
    });

    // house 1 
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 6.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: false,
	pivot: None
    });

    // house 1 (flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 6.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: false,
	pivot: None
    });


    // house 2 
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 7.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: false,
	pivot: None
    });

    // house 2 (flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 6.0 * 16.0,
	    y: 7.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: false,
	pivot: None
    });

    // dirt
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 8.0 * 16.0,
	    y: 9.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: false,
	pivot: None
    });
    
    // dirt (x-flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 8.0 * 16.0,
	    y: 9.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: false,
	pivot: None
    });
    
    // dirt (y-flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 8.0 * 16.0,
	    y: 9.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: false,
	flip_y: true,
	pivot: None
    });
    
    // dirt (xy-flipped)
    params.push(DrawTextureParams {
	dest_size: Some(Vec2{x: 32.0, y: 32.0}),
	source: Some(Rect{
	    x: 8.0 * 16.0,
	    y: 9.0 * 16.0,
	    w: 16.0,
	    h: 16.0}),
	rotation: 0.0,
	flip_x: true,
	flip_y: true,
	pivot: None
    });
    
    
    
    
    
    params
}	


pub struct ArenaMode {
    tiles: Texture2D,
    car_sprites: Vec<Texture2D>,
    params: Vec<DrawTextureParams>,
    tile_idx_arr: Vec<Vec<usize>>,
}



impl ArenaMode {
    pub fn new(tiles: Texture2D, car_sprites: Vec<Texture2D>) -> ArenaMode {
	tiles.set_filter(FilterMode::Nearest);
	
	ArenaMode {
	    tiles: tiles,
	    car_sprites: car_sprites,
	    params: make_params_vec(),
	    tile_idx_arr: vec!{vec!{ 0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  7},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  0},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  1},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  2},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  3},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  4},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  5},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  0,  2, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  6},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  2,  0,  1,  3,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  7},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  1,  3,  0,  2,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  0},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  2,  0, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  1},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  2},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  3},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  4},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  4,  5,  6,  7, 11,  8,  8,  8,  8,  8,  8,  9,  5},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  5,  4,  5,  4, 11,  8,  8,  8,  8,  8,  8,  9,  6},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  6,  4,  5,  6, 11,  8,  8,  8,  8,  8,  8,  9,  7},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  7,  4,  6,  5, 11,  8,  8,  8,  8,  8,  8,  9,  0},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  1},
			       vec!{ 0,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  9,  9,  8,  9, 10, 11,  8,  8,  8,  8,  8,  8,  9,  2},
			       vec!{ 0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  4,  5,  6,  7,  0,  1,  2,  3,  7},
	    },
	}
    }
}

impl GameMode for ArenaMode {
    fn get_name(&self) -> String {
	"ArenaMode".to_string()
    }

    fn init(&mut self) {

    }

    fn update(&mut self,
	      _dt_seconds: f32,
	      _canvas: &Canvas2D) -> Option<ModeTag> {
	None
    }

    fn draw(&self, _canvas:&Canvas2D) {

	clear_background(BLACK);
	
	for y in 0..self.tile_idx_arr.len() {
	    for x in 0..self.tile_idx_arr[y].len() {
		let tile_idx = self.tile_idx_arr[y][x];
		let param = &self.params[tile_idx];

		draw_texture_ex(self.tiles,
				32.0 * x as f32 + 6.0,
				32.0 * y as f32,
				WHITE,
				copy_params(&param));
	    }
	}

	let car_params = DrawTextureParams {
	    dest_size: Some(Vec2{x: 64.0,
				 y: 32.0}),
	    source: None,
	    rotation: 0.0,
	    flip_x: false,
	    flip_y: false,
	    pivot: None
	};


	let x_space = 96.0;
	let y_space = 64.0;
	
	draw_texture_ex(self.car_sprites[0],
			100.0 + 0.0 * x_space,
			100.0 + 0.0 * y_space,
			WHITE,
			copy_params(&car_params));

	draw_texture_ex(self.car_sprites[1],
			100.0 + 1.0 * x_space,
			100.0 + 0.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	draw_texture_ex(self.car_sprites[2],
			100.0 + 2.0 * x_space,
			100.0 + 0.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	draw_texture_ex(self.car_sprites[3],
			100.0 + 3.0 * x_space,
			100.0 + 0.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	draw_texture_ex(self.car_sprites[4],
			100.0 + 0.0 * x_space,
			100.0 + 1.0 * y_space,
			WHITE,
			copy_params(&car_params));

	draw_texture_ex(self.car_sprites[5],
			100.0 + 1.0 * x_space,
			100.0 + 1.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	draw_texture_ex(self.car_sprites[6],
			100.0 + 2.0 * x_space,
			100.0 + 1.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	draw_texture_ex(self.car_sprites[7],
			100.0 + 3.0 * x_space,
			100.0 + 1.0 * y_space,
			WHITE,
			copy_params(&car_params));
	
	
	/*

	for i in 0..9 {
	    
	    let param = &self.params[i];
	    
	    draw_texture_ex(self.tiles,
			    32.0 * i as f32, // x
			    0.0, // y
			    WHITE,
			    copy_params(&param));
	}
	 */			
    }
}
