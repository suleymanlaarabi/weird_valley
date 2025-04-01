use bevy::prelude::*;

#[derive(Resource)]
pub struct AppleRes {
    pub image: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}

#[derive(Resource)]
pub struct PlayerRes {
    pub image: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
}
