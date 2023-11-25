use bevy::{prelude::*, log::{self, LogPlugin}, window::PresentMode};
use wasm_bindgen::prelude::*;

use crate::examples::ecs::ECSExamplePlugin;

pub mod examples;
pub mod bridge;

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

pub fn pc_startup() {
  print!("----- pc startup ----");
  App::new()
      .add_plugins(DefaultPlugins)
      .add_plugins(ECSExamplePlugin)
      .run();
}

#[wasm_bindgen]
pub fn wasm_main() {
  web_startup();
}
