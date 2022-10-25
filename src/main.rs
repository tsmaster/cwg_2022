use gilrs::{Gilrs, Button, Event};
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;
use std::time::SystemTime;

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

mod mode;

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


    

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
	println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;
    

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
    

    let mut prev_time = SystemTime::now();
    let mut dt;
    
    loop {
	let cur_time = SystemTime::now();
	let last_frame_duration = prev_time.elapsed();

	if mode_mgr.get_current_mode_tag() == ModeTag::QuitMode {
	    break;
	}

	match last_frame_duration {
	    Ok(dur) => {dt = dur.as_millis() as f32 / 1000.0;},
	    Err(_) => continue,
	}

	//println!("elapsed {}", dt);
	
	let mut event_vec = vec!();
	
	// Examine new events
	while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);

	    event_vec.push(event);
	}
	
	if let Some(next_mode) =
	    mode_mgr.get_current_mode().update(dt,
					       event_vec,
					       &canvas) {
	    mode_mgr.set_current_mode(next_mode);
	    continue;
	}

	// You can also use cached gamepad state
	if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
            if gamepad.is_pressed(Button::South) {
		println!("Button South is pressed (XBox - A, PS - X)");
            }
	}

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
