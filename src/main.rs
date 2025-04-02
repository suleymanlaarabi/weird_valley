use animation_plugin::{Animation2d, CustomAnimationPlugin};
use ecs_common::prelude::*;
use fruvibe_ui::WeirdTopLeftContainer;
use game_resources::{
    GameResourcePlugin,
    ressource::{AppleRes, PlayerRes},
};
use interact_plugin::{
    InteractablePlugin,
    components::{CanInteract, Interactable},
};
use inventory_ui::components::InventoryUi;
use plugins::{game::GamePlugin, interaction::WeirdInteractionPlugin, player::PlayerPlugin};
use prelude::*;

use velox2d::{
    Velox2dPlugin,
    prelude::{VeloxController, VeloxControllerInputConfig, VeloxMovement},
};
use zestalive::ZestalivePlugin;

pub mod common;
pub mod plugins;
pub mod prelude;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "weird".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins((
            GamePlugin,
            PlayerPlugin,
            WeirdInteractionPlugin,
            InteractablePlugin,
            PhysicsPlugins::default(),
            Velox2dPlugin,
            GameResourcePlugin,
            CustomAnimationPlugin,
            ZestalivePlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(player_res: Res<PlayerRes>, apple_res: Res<AppleRes>, mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite {
            image: apple_res.image.clone(),
            texture_atlas: apple_res.atlas.to_texture_atlas().to_some_value(),
            ..default()
        },
        Interactable,
        Collider::rectangle(40., 40.),
        Transform::from_xyz(10., 10., 0.).with_scale(Vec3::splat(3.)),
        Animation2d::new(0.2, 0, 3),
    ));

    commands.spawn((
        VeloxController,
        CanInteract,
        VeloxControllerInputConfig::default(),
        VeloxMovement { speed: 200. },
        Collider::rectangle(40., 40.),
        Sprite {
            image: player_res.image.clone(),
            texture_atlas: player_res.atlas.to_texture_atlas().to_some_value(),
            ..default()
        },
        Animation2d::new(0.2, 0, 3),
        Transform::from_xyz(10., 10., 1.).with_scale(Vec3::splat(3.)),
    ));
}
