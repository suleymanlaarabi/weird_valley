use systems::handle_inventory_toggle;

use crate::prelude::*;

pub mod components;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_inventory_toggle);
    }
}
