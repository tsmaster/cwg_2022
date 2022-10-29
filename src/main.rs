//use gilrs::{Gilrs, Button, Event};
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;

use crate::mode::ModeMgr;
use crate::mode::ModeTag;
use crate::mode::about_mode::AboutMode;
use crate::mode::bdg_mode::BdgMode;
use crate::mode::credits_mode::CreditsMode;
use crate::mode::cwg_title_mode::CwgTitleMode;
use crate::mode::menu_mode::MenuMode;
use crate::mode::new_game_mode::NewGameMode;
use crate::mode::play_mode::PlayMode;
use crate::mode::quit_mode::QuitMode;
use crate::mode::settings_mode::SettingsMode;
use crate::mode::arena_mode::ArenaMode;


mod mode;
mod text;

const WIDTH: f32 = 1200_f32;
const HEIGHT: f32 = 675_f32;

fn window_conf() -> Conf {
    Conf {
	window_title: "CWG 2022".to_owned(),
	window_width: WIDTH as i32,
	window_height: HEIGHT as i32,
	fullscreen: false,
	..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    println!("Hello, Cars With(?) Guns 2022!");

    let canvas = Canvas2D::new(WIDTH, HEIGHT);

    let bdg_screen = load_texture("assets/bdg_screen.png")
	.await
	.unwrap();

    let title_screen = load_texture("assets/cwg_screen.png")
	.await
	.unwrap();

    let menu_screen = load_texture("assets/menu_screen.png")
	.await
	.unwrap();

    let credits_screen = load_texture("assets/credits_screen.png")
	.await
	.unwrap();

    let new_game_screen = load_texture("assets/new_game_screen.png")
	.await
	.unwrap();

    let play_screen = load_texture("assets/play_screen.png")
	.await
	.unwrap();

    let quit_screen = load_texture("assets/quit_screen.png")
	.await
	.unwrap();

    let settings_screen = load_texture("assets/settings_screen.png")
	.await
	.unwrap();

    let about_screen = load_texture("assets/about_screen.png")
	.await
	.unwrap();

    let pixel_shmup_tiles = load_texture("assets/Kenney/pixel_shmup_tiles_packed.png")
	.await
	.unwrap();

    let car_0_sprite = load_texture("assets/car_0.png")
	.await
	.unwrap();

    let car_1_sprite = load_texture("assets/car_1.png")
	.await
	.unwrap();

    let car_2_sprite = load_texture("assets/car_2.png")
	.await
	.unwrap();

    let car_3_sprite = load_texture("assets/car_3.png")
	.await
	.unwrap();

    let car_4_sprite = load_texture("assets/car_4.png")
	.await
	.unwrap();

    let car_5_sprite = load_texture("assets/car_5.png")
	.await
	.unwrap();

    let car_6_sprite = load_texture("assets/car_6.png")
	.await
	.unwrap();

    let car_7_sprite = load_texture("assets/car_7.png")
	.await
	.unwrap();

    let car_sprites = vec!{car_0_sprite,
			   car_1_sprite,
			   car_2_sprite,
			   car_3_sprite,
			   car_4_sprite,
			   car_5_sprite,
			   car_6_sprite,
			   car_7_sprite};

    /*

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
	println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;
    */

    let mut mode_mgr = ModeMgr::new(ModeTag::BdgMode);

    mode_mgr.register_mode(ModeTag::BdgMode,
			   Box::new(BdgMode::new(bdg_screen)));

    mode_mgr.register_mode(ModeTag::CwgTitleMode,
			   Box::new(CwgTitleMode::new(title_screen)));

    mode_mgr.register_mode(ModeTag::MenuMode,
			   Box::new(MenuMode::new(menu_screen)));

    mode_mgr.register_mode(ModeTag::CreditsMode,
			   Box::new(CreditsMode::new(credits_screen)));
    
    mode_mgr.register_mode(ModeTag::AboutMode,
			   Box::new(AboutMode::new(about_screen)));
    
    mode_mgr.register_mode(ModeTag::NewGameMode,
			   Box::new(NewGameMode::new(new_game_screen)));
    
    mode_mgr.register_mode(ModeTag::PlayMode,
			   Box::new(PlayMode::new(play_screen)));
    
    mode_mgr.register_mode(ModeTag::QuitMode,
			   Box::new(QuitMode::new(quit_screen)));
    
    mode_mgr.register_mode(ModeTag::SettingsMode,
			   Box::new(SettingsMode::new(settings_screen)));

    mode_mgr.register_mode(ModeTag::ArenaMode,
			   Box::new(ArenaMode::new(pixel_shmup_tiles,
						   car_sprites)));
    

    let mut prev_time = macroquad::time::get_time();
    let mut dt: f32;
    
    loop {
	let cur_time = macroquad::time::get_time();
	dt = (cur_time - prev_time) as f32;

	if mode_mgr.get_current_mode_tag() == ModeTag::QuitMode {
	    break;
	}

/*
	let mut event_vec = vec!();

	// Examine new events
	while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);

	    event_vec.push(event);
	}
*/

	if let Some(next_mode) =
	    mode_mgr.get_current_mode().update(dt,
					       //event_vec,
					       &canvas) {
	    mode_mgr.set_current_mode(next_mode);
	}

	/*
	// You can also use cached gamepad state
	if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
		println!("Button South is pressed (XBox - A, PS - X)");
            }
	}*/

	// macroquad(canvas) 

	set_camera(&canvas.camera);

	clear_background(WHITE);

	// Draw inside canvas...

	//draw_rectangle(0.0, 0.0, 60.0, 60.0, RED);

	/*
	draw_texture(
	    title_screen,
	    0.0,
	    0.0,
	    WHITE,
	);*/

	draw_text("CWG 2022", 20.0, 20.0, 30.0, DARKGRAY);

	mode_mgr.get_current_mode().draw(&canvas);
	
	draw_text(&mode_mgr.get_current_mode().get_name(), 20.0, 50.0, 30.0, RED);


	set_default_camera();
	
	canvas.draw();

	prev_time = cur_time;
	
	next_frame().await
    }
}
