use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct IWeird {
    pub marchand_value: f32,
}

impl IWeird {
    pub fn new(marchand_value: f32) -> Self {
        Self { marchand_value }
    }
}
