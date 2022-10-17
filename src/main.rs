use gilrs::{Gilrs, Button, Event};
use macroquad::prelude::*;
use macroquad_canvas::Canvas2D;


#[macroquad::main("CWG 2022")]
async fn main() {
    println!("Hello, Cars With(?) Guns 2022!");

    let canvas = Canvas2D::new(800_f32, 600_f32);
    

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

	set_default_camera();

	canvas.draw();

	next_frame().await
    }
}
