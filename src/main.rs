use gilrs::{Gilrs, Button, Event};
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;


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

    let mut gilrs = Gilrs::new().unwrap();

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
	println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    let mut active_gamepad = None;

    loop {
	// Examine new events
	while let Some(Event { id, event, time }) = gilrs.next_event() {
            println!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);
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

	draw_rectangle(0.0, 0.0, 60.0, 60.0, RED);

	draw_texture(
	    title_screen,
	    0.0,
	    0.0,
	    WHITE,
	);

	draw_text("CWG 2022", 20.0, 20.0, 30.0, DARKGRAY);

	set_default_camera();

	canvas.draw();

	next_frame().await
    }
}
