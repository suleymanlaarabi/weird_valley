use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Collider, CollidingEntities)]
pub struct CanInteract;

#[derive(Component)]
pub struct Interactable;

#[derive(Component)]
pub struct InteractionAvailable;
