use crate::{components, prelude::*};

#[derive(Component)]
pub struct PlayerInventory;

#[derive(Component)]
#[require(
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    }
)]
pub struct PlayerUi;

components!(Player);
