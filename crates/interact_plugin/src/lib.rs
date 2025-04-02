use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, process_interactable);
    }
}
