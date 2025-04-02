use bevy::{prelude::*, window::PrimaryWindow};

pub fn despawn_all<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn set_cursor_visibility<const T: bool>(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.single_mut().expect("Window not available");

    window.cursor_options.visible = T;
}

#[macro_export]
macro_rules! set_state {
    ($value:expr) => {
        |mut states: ResMut<NextState<_>>| {
            states.set($value);
        }
    };
}

#[macro_export]
macro_rules! set_state_system {
    ($value:expr) => {
        |_: Trigger<Pointer<Click>>, mut states: ResMut<NextState<_>>| {
            states.set($value);
        }
    };
}
