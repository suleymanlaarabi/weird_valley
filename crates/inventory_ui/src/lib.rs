use bevy::prelude::*;

pub mod components;
mod systems;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {}
}
