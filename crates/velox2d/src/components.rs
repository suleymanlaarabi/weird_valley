use bevy::prelude::*;

#[derive(Component)]
pub struct VeloxController;

#[derive(Component)]
pub struct VeloxMovement {
    pub speed: f32,
}

#[derive(Component)]
pub struct VeloxControllerInputConfig {
    pub top: KeyCode,
    pub bottom: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
}

impl Default for VeloxControllerInputConfig {
    fn default() -> Self {
        Self {
            top: KeyCode::KeyW,
            bottom: KeyCode::KeyS,
            right: KeyCode::KeyD,
            left: KeyCode::KeyA,
        }
    }
}
