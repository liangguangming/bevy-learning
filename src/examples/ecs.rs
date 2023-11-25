use bevy::prelude::*;
pub struct ECSExamplePlugin;

#[derive(Component, Debug)]
struct Velocity { x: f32, y: f32 }

#[derive(Component, Debug)]
struct Position { x: f32, y: f32 }

fn spawn_spaceship(mut commands: Commands) {
  // create a new entity with the given component
  let spaceship = (Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 });
  bevy::log::info!("create a spaceship {:?}", spaceship);
  commands.spawn(spaceship);
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>) {
  for (velocity, mut position) in query.iter_mut() {
      position.x += velocity.x;
      position.y += velocity.y;
  }
}

fn print_position(query: Query<(Entity, &Position)>) {
  for (entity, position) in query.iter() {
    bevy::log::info!("Entity {:?} is at position {:?}", entity, position);
  }
}

impl Plugin for ECSExamplePlugin {
    fn build(&self, app: &mut App) {
      app
        .add_systems(Startup, &spawn_spaceship)
        .add_systems(Update, (&update_position, &print_position));
    }
}