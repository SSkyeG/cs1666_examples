use bevy::{
	prelude::*,
	window::PresentMode,
};

const TITLE: &str = "Key Events";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::CYAN))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(keyboard_input)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());
}

fn keyboard_input(input: Res<Input<KeyCode>>, mut clear_color: ResMut<ClearColor>) {
    if input.just_pressed(KeyCode::W) {
		clear_color.0 = Color::RED;
    }
    else if input.just_pressed(KeyCode::S) {
		clear_color.0 = Color::BLUE;
    }
	else if input.just_pressed(KeyCode::A) {
		clear_color.0 = Color::GREEN;
    }
	else if input.just_pressed(KeyCode::D) {
		clear_color.0 = Color::FUCHSIA;
    }
}

//TODO: Write a system to change the clear color in response to a keypress
//      * w -> red
//      * a -> green
//      * s -> blue
//      * d -> fuchsia
// Don't forget to add your system to the app in `main()`!

//<Your code here>
