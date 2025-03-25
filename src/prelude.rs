pub use bevy::prelude::*;

#[macro_export]
macro_rules! private_component {
    ($name:ident) => {
        #[derive(bevy::ecs::component::Component)]
        struct $name;
    };
}

#[macro_export]
macro_rules! component {
    ($name:ident) => {
        #[derive(bevy::ecs::component::Component, Default, Clone, Debug)]
        pub struct $name;
    };
}

#[macro_export]
macro_rules! components {
    ($($name:ident),*) => {
        $(
            component!($name);
        )*
    };
}
