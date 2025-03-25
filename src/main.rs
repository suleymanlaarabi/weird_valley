use bevy::window::{CursorGrabMode, CursorOptions};
use prelude::*;

pub mod common;
pub mod plugins;
pub mod prelude;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "game".to_string(),
                cursor_options: CursorOptions {
                    grab_mode: CursorGrabMode::Confined,
                    visible: true,
                    ..default()
                },
                focused: true,
                // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .run();
}
