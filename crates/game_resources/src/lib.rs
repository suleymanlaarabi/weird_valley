use bevy::prelude::*;
use ressource::{AppleRes, PlayerRes};

pub mod ressource;

pub struct GameResourcePlugin;

impl Plugin for GameResourcePlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();

        insert_zestalive_assets(world);
    }
}

fn insert_zestalive_assets(world: &mut World) {
    let res = world.get_resource_mut::<AssetServer>().unwrap();

    let apple_sprite: Handle<Image> = res.load("fruits_sprites/apple_sprite.png");
    let apple_atlas = res.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4,
        1,
        None,
        None,
    ));

    let player_sprite: Handle<Image> = res.load("character/character_sprite.png");
    let player_atlas = res.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 32),
        4,
        2,
        None,
        None,
    ));

    world.insert_resource(PlayerRes {
        atlas: player_atlas,
        image: player_sprite,
    });
    world.insert_resource(AppleRes {
        atlas: apple_atlas,
        image: apple_sprite,
    });
}
