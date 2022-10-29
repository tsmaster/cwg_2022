// mode/arena_mode.rs
//
//

//use gilrs::EventType;
//use gilrs::EventType::ButtonChanged;
//use gilrs::EventType::ButtonPressed;
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::math::vectors::Vec2f;
use crate::math::utils::degrees_to_radians;
use crate::mode::GameMode;
use crate::mode::ModeTag;
use crate::mode::car::Car;
use crate::mode::car::CarControls;

#[derive(Debug, Clone, Copy)]
enum CarControlMode {
    PlayerKBControlArrows,
    PlayerKBControlWasd,
    PlayerMouseControl,
    //PlayerGamepadControl,
    AiRandomWalk
}

struct CarBinding {
    texture_params: DrawTextureParams,
    physics: Car,
    control_mode: CarControlMode,
    sprite: Texture2D,
}

impl CarBinding {
    fn update(&mut self, dt: f32) {
	let control_input = match self.control_mode {
	    CarControlMode::PlayerKBControlArrows => self.get_kb_arrow_control_input(),
	    CarControlMode::PlayerKBControlWasd => self.get_kb_wasd_control_input(),
	    CarControlMode::PlayerMouseControl => self.get_mouse_control_input(),
	    CarControlMode::AiRandomWalk => self.get_ai_random_walk_control_input()
	};

	self.physics.update(dt, &control_input);
    }

    fn draw(&self) {
	// println!("drawing at {:?}", self.physics.position);

	let mut tex_params = copy_params(&self.texture_params);
	tex_params.rotation = degrees_to_radians(self.physics.heading_degrees);
	
	draw_texture_ex(self.sprite,
			self.physics.position.x,
			self.physics.position.y,
			WHITE,
			tex_params);
    }

    fn get_kb_wasd_control_input(&self) -> CarControls {
	CarControls {
	    steering: 0.0,
	    acceleration: 0.0,
	    brake: 0.0
	}
    }

    fn get_kb_arrow_control_input(&self) -> CarControls {
	let s = if is_key_down(KeyCode::Left) {
	    -1.0
	} else {
	    if is_key_down(KeyCode::Right) {
		1.0
	    } else {
		0.0
	    }
	};

	CarControls {
	    steering: s,
	    acceleration: if is_key_down(KeyCode::Up) { 1.0 } else { 0.0 },
	    brake: if is_key_down(KeyCode::Down) { 1.0 } else {0.0}
	}
    }
    
    fn get_mouse_control_input(&self) -> CarControls {
	CarControls {
	    steering: 0.0,
	    acceleration: 0.0,
	    brake: 0.0
	}
    }
    
    fn get_ai_random_walk_control_input(&self) -> CarControls {
	let throttle = macroquad::rand::gen_range(-1.0, 1.0);

	let acc = f32::max(throttle, 0.0);
	let brk = -1.0 * f32::min(0.0, throttle);
	
	CarControls {
	    steering: macroquad::rand::gen_range(-1.0, 1.0),
	    acceleration: acc,
	    brake: brk
	}
    }
    
    
}

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
    cars: Vec<CarBinding>,
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
	    cars: vec!(),
	}
    }
}

impl GameMode for ArenaMode {
    fn get_name(&self) -> String {
	"ArenaMode".to_string()
    }

    fn init(&mut self) {
	self.cars.clear();

	self.cars.push(CarBinding {
	    texture_params: DrawTextureParams {
		dest_size: Some(Vec2 {x: 64.0,
				      y: 32.0}),
		source: None,
		rotation: 0.0,
		flip_x: false,
		flip_y: false,
		pivot: None
	    },
	    physics: Car::new(&Vec2f::new(100.0, 100.0),
			      0.0),
	    control_mode: CarControlMode::AiRandomWalk,
	    sprite: self.car_sprites[0],
	});

	self.cars.push(CarBinding {
	    texture_params: DrawTextureParams {
		dest_size: Some(Vec2 {x: 64.0,
				      y: 32.0}),
		source: None,
		rotation: 0.0,
		flip_x: false,
		flip_y: false,
		pivot: None
	    },
	    physics: Car::new(&Vec2f::new(100.0, 160.0),
			      0.0),
	    control_mode: CarControlMode::PlayerKBControlArrows,
	    sprite: self.car_sprites[1],
	});
    }

    fn update(&mut self,
	      dt_seconds: f32,
	      _canvas: &Canvas2D) -> Option<ModeTag> {

	if is_key_pressed(KeyCode::Escape) {
	    return Some(ModeTag::MenuMode);
	}

	for mut car in self.cars.iter_mut() {
	    car.update(dt_seconds);
	}
	
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

	/*
	let car_params = DrawTextureParams {
	    dest_size: Some(Vec2{x: 64.0,
				 y: 32.0}),
	    source: None,
	    rotation: 0.0,
	    flip_x: false,
	    flip_y: false,
	    pivot: None
	};*/


	/*
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

	 */

	for car in &self.cars {
	    car.draw();
	}
	
	
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
