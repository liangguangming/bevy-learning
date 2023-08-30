use bevy::{prelude::*, log::{self, LogPlugin}, window::PresentMode};

use crate::bridge::message::send_msg_to_js;

fn hello_world_system() {
  log::info!("hello world!!!\n");
}

static mut a: i32 = 1;
fn loop_send_msg_to_web() {
    unsafe {
        a += 1;
        send_msg_to_js(format!("msg from bevy: {}", a).as_str());
    }
}

pub fn main() {
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
    }))
    // .add_systems(Update, &hello_world_system)
    // .add_systems(Update, &loop_send_msg_to_web)
    .run();
}