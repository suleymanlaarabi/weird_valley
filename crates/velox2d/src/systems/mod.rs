use crate::prelude::*;
use animation_plugin::Animation2d;
use bevy::prelude::*;

pub fn movement_system(
    mut query: Query<
        (
            &VeloxControllerInputConfig,
            &VeloxMovement,
            &mut Transform,
            Option<&mut Animation2d>,
        ),
        With<VeloxController>,
    >,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();
    for (controller_input_config, movement_config, mut transform, animation) in &mut query {
        let mut direction = Vec3::ZERO;
        if keys.pressed(controller_input_config.top) {
            direction.y += 1.;
        }
        if keys.pressed(controller_input_config.bottom) {
            direction.y -= 1.;
        }
        if keys.pressed(controller_input_config.left) {
            direction.x -= 1.;
        }
        if keys.pressed(controller_input_config.right) {
            direction.x += 1.;
        }
        if let Some(mut animation) = animation {
            if direction.y > 0. {
                animation.set_animation(4, 7, 0.2);
            } else {
                animation.set_animation(0, 3, 0.2);
            }
            if direction == Vec3::ZERO {
                animation.set_animation(0, 1, 0.4);
            }
        }
        transform.translation += direction * movement_config.speed * delta;
    }
}
