// Components: Rust structs that implement the Component trait
use bevy::ecs::component::Component;

/*
 * Components are the data associated with entities.
 * To create a new component type, simply define a Rust struct or enum, and derive the Component trait.
 */

#[derive(Component)]
pub struct MapSize {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Component)]
pub struct MapName(pub String);
