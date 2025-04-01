use animation_plugin::{Animation2d, CustomAnimationPlugin};
use bevy::window::{CursorGrabMode, CursorOptions};
use ecs_common::prelude::*;
use game_resources::{
    GameResourcePlugin,
    ressource::{AppleRes, PlayerRes},
};
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
                        cursor_options: CursorOptions {
                            grab_mode: CursorGrabMode::Confined,
                            visible: true,
                            ..default()
                        },
                        focused: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
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
        Transform::from_xyz(10., 10., 0.).with_scale(Vec3::splat(3.)),
        Animation2d::new(0.2, 0, 3),
    ));

    commands.spawn((
        VeloxController,
        VeloxControllerInputConfig::default(),
        VeloxMovement { speed: 200. },
        Sprite {
            image: player_res.image.clone(),
            texture_atlas: player_res.atlas.to_texture_atlas().to_some_value(),
            ..default()
        },
        Animation2d::new(0.2, 0, 3),
        Transform::from_xyz(10., 10., 1.).with_scale(Vec3::splat(3.)),
    ));
}
