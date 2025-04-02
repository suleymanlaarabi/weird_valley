use avian2d::prelude::*;
use bevy::{ecs::entity::EntityHashSet, prelude::*};

use crate::components::{CanInteract, Interactable, InteractionAvailable};

pub fn process_interactable(
    query_can_interact: Query<
        (Entity, &CollidingEntities, Option<&InteractionAvailable>),
        With<CanInteract>,
    >,
    query_interactable: Query<Entity, With<Interactable>>,
    mut commands: Commands,
) {
    let entity_hashset: EntityHashSet = query_interactable.iter().collect();

    for (entity, colliding_entities, interaction_available) in &query_can_interact {
        let entities: &EntityHashSet = &colliding_entities.0;
        if interaction_available.is_none() {
            if !entity_hashset.is_disjoint(entities) {
                commands.entity(entity).insert(InteractionAvailable);
            }
        } else {
            if entity_hashset.is_disjoint(entities) {
                commands.entity(entity).remove::<InteractionAvailable>();
            }
        }
    }
}
