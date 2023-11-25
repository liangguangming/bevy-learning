use bevy::{prelude::*, log::{self, LogPlugin}, window::PresentMode};

pub fn web_startup() {
    App::new()
    .add_plugins(DefaultPlugins.set(LogPlugin {
        filter: "info,wgpu_core=warn,bevy_render=warn".into(),
        level: bevy::log::Level::INFO,
    })
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                canvas: Some(String::from("#xspiral")),
                resizable: true,
                ..default()
            }),
            ..default()
        })
    )
    // .add_systems(Update, &hello_world_system)
    .run();
}

fn main() {
    print!("----- pc startup ----");
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}