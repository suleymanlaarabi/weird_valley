use fruvibe_ui::{
    WeirdCenteredContainer,
    color::{BACKGROUND_COLOR, BACKGROUND_COLOR_TRANSPARENT},
};
use inventory_ui::components::InventoryUi;

use crate::prelude::*;

use super::player::components::{Player, PlayerUi};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, PlayerUi)).with_children(|player| {
        player
            .spawn(WeirdCenteredContainer)
            .with_children(|parent| {
                parent.spawn((InventoryUi, BackgroundColor(BACKGROUND_COLOR_TRANSPARENT)));
            });
    });
}
