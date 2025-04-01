use bevy::prelude::*;
use iweird::components::IWeird;

#[derive(Component, Clone)]
pub struct Zestalive {
    pub item: IWeird,
    pub production_time: f32,
    pub zestalive_type: ZestaliveType,
}

impl Zestalive {
    pub fn from_type(zestalive_type: ZestaliveType) -> Self {
        match zestalive_type {
            ZestaliveType::Apple => Zestalive {
                zestalive_type: zestalive_type.clone(),
                production_time: 1.,
                item: IWeird::new(1.),
            },
        }
    }
}

#[derive(Clone)]
pub enum ZestaliveType {
    Apple,
}

#[derive(Event)]
pub struct ZestaliveSpawnEvent {
    pub zestalive: Zestalive,
}
