use bevy::prelude::*;

pub trait ToSomeValue<T> {
    fn to_some_value(self) -> Option<T>;
}

impl<T> ToSomeValue<T> for T {
    fn to_some_value(self) -> Option<T> {
        Some(self)
    }
}

pub trait ToTextureAtlas {
    fn to_texture_atlas(&self) -> TextureAtlas;
}

impl ToTextureAtlas for Handle<TextureAtlasLayout> {
    fn to_texture_atlas(&self) -> TextureAtlas {
        TextureAtlas {
            layout: self.clone(),
            index: 0,
        }
    }
}
