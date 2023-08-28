use bevy::{prelude::*, log::{self, LogPlugin}, window::PresentMode};
use wasm_bindgen::prelude::*;

fn hello_world_system() {
    log::info!("hello world!!!\n");
}

#[wasm_bindgen]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,bevy_render=info".into(),
            level: bevy::log::Level::DEBUG,
        }).set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                canvas: Some(String::from("#xspiral")),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Update, &hello_world_system)
        .run();
}
