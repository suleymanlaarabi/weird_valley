use bevy::prelude::*;
use fruvibe_ui::node_builder::*;

#[derive(Component)]
pub struct InventoryItemUi;

#[derive(Component)]
#[require(
    Node = Node::node_builder().width(200.).height(200.).build(),
    BackgroundColor(Color::srgb_u8(10, 10, 10)),
    BorderRadius = BorderRadius::all(Val::Px(10.))
)]
pub struct InventoryUi;

#[derive(Event)]
pub enum InvetoryEvent {
    Open,
    Close,
}
