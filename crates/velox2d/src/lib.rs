use bevy::prelude::*;
use systems::movement_system;

pub mod components;
pub mod event;
pub mod prelude;
mod systems;

pub struct Velox2dPlugin;

impl Plugin for Velox2dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement_system);
    }
}
