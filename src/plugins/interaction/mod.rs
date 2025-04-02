use bevy::prelude::*;
use fruvibe_ui::{
    WeirdBottomCenterContainer, WeirdButton, color::BACKGROUND_COLOR, default_button_node,
    node_builder::AsNodeBuilder,
};
use interact_plugin::components::InteractionAvailable;

pub struct WeirdInteractionPlugin;

#[derive(Component)]
pub struct WeirdInteractionUi;

impl Plugin for WeirdInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_interaction_ui);
        app.add_systems(FixedUpdate, display_interaction);
    }
}

fn display_interaction(
    query: Query<(), With<InteractionAvailable>>,
    mut query_ui: Query<&mut Node, With<WeirdInteractionUi>>,
) {
    if query.is_empty() {
        for mut node in &mut query_ui {
            node.display = Display::None;
        }
    } else {
        for mut node in &mut query_ui {
            node.display = Display::Flex;
        }
    }
}

fn spawn_interaction_ui(mut commands: Commands) {
    commands
        .spawn((WeirdBottomCenterContainer, WeirdInteractionUi))
        .with_children(|ui| {
            ui.spawn((
                WeirdButton,
                BackgroundColor(BACKGROUND_COLOR),
                default_button_node().as_node_builder().width(250.).build(),
                BorderRadius::all(Val::Px(10.)),
            ))
            .with_child((Text::new("Press E to interact"), TextColor(Color::BLACK)));
        });
}
