use bevy::prelude::*;
use node_builder::AsNodeBuilder;

pub mod color;
pub mod node_builder;

pub fn top_left_container() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(10.)),
        ..default()
    }
}

pub fn bottom_center_container() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::End,
        ..default()
    }
}

pub fn centered_container() -> Node {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(Val::Px(10.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

#[derive(Component)]
#[require(
    Node = centered_container()
)]
pub struct WeirdCenteredContainer;

#[derive(Component)]
#[require(
    Node = top_left_container()
)]
pub struct WeirdTopLeftContainer;

#[derive(Component)]
#[require(
    Node = bottom_center_container()
)]
pub struct WeirdBottomCenterContainer;

pub fn default_button_node() -> Node {
    Node::node_builder()
        .width(100.)
        .height(50.)
        .padding(10.)
        .flex_center()
        .build()
}

#[derive(Component)]
#[require(
    Node = default_button_node(),
    BackgroundColor(Color::srgb_u8(255, 255, 255)),
    Button
)]
pub struct WeirdButton;
