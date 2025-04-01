use bevy::prelude::*;

pub mod components;
mod systems;

pub struct ZestalivePlugin;

impl Plugin for ZestalivePlugin {
    fn build(&self, _app: &mut App) {}
}
