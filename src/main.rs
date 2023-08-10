use bevy::{prelude::*, window::PresentMode};
mod camera;
mod heart;

fn main() {
    App::new()
        // window clear color
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // window origin size
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                resolution: (400., 400.).into(),
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(heart::HeartPlugin)
        // .add_plugins(snake::SnakePlugin)
        .run();
}
